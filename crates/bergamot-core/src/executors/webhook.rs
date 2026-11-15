use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use reqwest::{
    header::{HeaderName, HeaderValue, AUTHORIZATION},
    multipart::{Form, Part},
    redirect::Policy,
    Client, ClientBuilder, Method, Url,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

use crate::descriptor::ExecutorConfig;

use super::{ExecutionError, ExecutionOutput, MAX_CAPTURED_OUTPUT_BYTES, MAX_WEBHOOK_FILE_BYTES};

const SECRET_PLACEHOLDER: &str = "${secret}";

/// Retrieves secrets referenced by webhook action manifests.
pub trait SecretProvider: Send + Sync {
    fn get_secret(&self, key: &str) -> Result<String, ExecutionError>;
}

pub use crate::secrets::KeyringSecretStore as KeyringSecretProvider;

#[derive(Clone, Copy)]
pub(crate) enum ResponseTarget<'a> {
    Capture,
    Discard,
    File(&'a Path),
}

impl<T: crate::SecretStore> SecretProvider for T {
    fn get_secret(&self, key: &str) -> Result<String, ExecutionError> {
        crate::SecretStore::get_secret(self, key)
            .map_err(|error| ExecutionError::Secret(error.to_string()))?
            .ok_or_else(|| ExecutionError::Secret("credential has not been configured".into()))
    }
}

/// Sends files to a multipart HTTP endpoint without exposing credentials to redirects.
pub struct WebhookExecutor {
    client: Client,
    secrets: Arc<dyn SecretProvider>,
}

impl WebhookExecutor {
    pub fn new(secrets: Arc<dyn SecretProvider>) -> Result<Self, ExecutionError> {
        Self::with_client_builder(secrets, Client::builder())
    }

    /// Applies Bergamot's redirect policy to a caller-configured HTTP client builder.
    pub fn with_client_builder(
        secrets: Arc<dyn SecretProvider>,
        builder: ClientBuilder,
    ) -> Result<Self, ExecutionError> {
        let client = builder
            .redirect(Policy::none())
            .build()
            .map_err(|_| ExecutionError::Webhook("could not initialize HTTP client".into()))?;

        Ok(Self { client, secrets })
    }

    /// A header value can reference `credentialKey` through `${secret}`.
    /// Without such a header or an explicit Authorization header, the credential is a bearer token.
    pub async fn execute(
        &self,
        executor: &ExecutorConfig,
        files: &[PathBuf],
        cancel: &CancellationToken,
    ) -> Result<ExecutionOutput, ExecutionError> {
        self.execute_with_response(executor, files, ResponseTarget::Capture, cancel)
            .await
    }

    pub(crate) async fn execute_with_response(
        &self,
        executor: &ExecutorConfig,
        files: &[PathBuf],
        target: ResponseTarget<'_>,
        cancel: &CancellationToken,
    ) -> Result<ExecutionOutput, ExecutionError> {
        let ExecutorConfig::Webhook {
            method,
            url,
            headers,
            file_field_name,
            credential_key,
            timeout_seconds,
        } = executor
        else {
            return Err(ExecutionError::InvalidConfig(
                "webhook executor requires webhook configuration".into(),
            ));
        };

        if files.is_empty() {
            return Err(ExecutionError::InvalidConfig(
                "webhook executor requires at least one file".into(),
            ));
        }
        if file_field_name.trim().is_empty() {
            return Err(ExecutionError::InvalidConfig(
                "webhook file field name cannot be empty".into(),
            ));
        }
        if *timeout_seconds == 0 {
            return Err(ExecutionError::InvalidConfig(
                "webhook timeout must be greater than zero".into(),
            ));
        }

        let method = Method::from_bytes(method.as_bytes())
            .map_err(|_| ExecutionError::InvalidConfig("invalid webhook HTTP method".into()))?;
        let url = Url::parse(url)
            .map_err(|_| ExecutionError::InvalidConfig("invalid webhook URL".into()))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(ExecutionError::InvalidConfig(
                "webhook URL must be HTTP or HTTPS and cannot contain credentials".into(),
            ));
        }

        let future = self.send_request(
            self.client.request(method, url),
            headers,
            file_field_name,
            files,
            credential_key.as_deref(),
            target,
        );
        let timeout = Duration::from_secs(u64::from(*timeout_seconds));
        tokio::select! {
            biased;
            _ = cancel.cancelled() => Err(ExecutionError::Cancelled),
            result = tokio::time::timeout(timeout, future) => match result {
                Ok(result) => result,
                Err(_) => Err(ExecutionError::TimedOut(*timeout_seconds)),
            },
        }
    }

    async fn send_request(
        &self,
        mut request: reqwest::RequestBuilder,
        headers: &BTreeMap<String, String>,
        file_field_name: &str,
        files: &[PathBuf],
        credential_key: Option<&str>,
        target: ResponseTarget<'_>,
    ) -> Result<ExecutionOutput, ExecutionError> {
        let header_uses_secret = headers
            .values()
            .any(|value| value.contains(SECRET_PLACEHOLDER));
        let has_authorization = headers
            .keys()
            .any(|name| name.eq_ignore_ascii_case(AUTHORIZATION.as_str()));
        if header_uses_secret && credential_key.is_none() {
            return Err(ExecutionError::InvalidConfig(
                "a credential key is required by webhook secret placeholders".into(),
            ));
        }

        let mut form = Form::new();
        for path in files {
            form = form.part(file_field_name.to_owned(), file_part(path).await?);
        }

        let secret = match credential_key {
            Some(key) if header_uses_secret || !has_authorization => self.get_secret(key).await?,
            _ => String::new(),
        };

        for (name, value) in headers {
            let name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| ExecutionError::InvalidConfig("invalid webhook header name".into()))?;
            let uses_secret = value.contains(SECRET_PLACEHOLDER);
            let interpolated = value.replace(SECRET_PLACEHOLDER, &secret);
            let mut value = HeaderValue::from_str(&interpolated).map_err(|_| {
                ExecutionError::InvalidConfig("invalid webhook header value".into())
            })?;
            value.set_sensitive(uses_secret || name == AUTHORIZATION);
            request = request.header(name, value);
        }

        if !header_uses_secret && !has_authorization && credential_key.is_some() {
            request = request.bearer_auth(&secret);
        }

        let response = request
            .multipart(form)
            .send()
            .await
            .map_err(|_| ExecutionError::Webhook("webhook request failed".into()))?;
        let status = response.status();
        if !status.is_success() {
            return Err(ExecutionError::Webhook(format!(
                "webhook returned HTTP status {status}"
            )));
        }

        let stdout = consume_response(response, target).await?;

        Ok(ExecutionOutput {
            stdout,
            stderr: Vec::new(),
        })
    }

    async fn get_secret(&self, key: &str) -> Result<String, ExecutionError> {
        let provider = Arc::clone(&self.secrets);
        let key = key.to_owned();
        let secret = tokio::task::spawn_blocking(move || provider.get_secret(&key))
            .await
            .map_err(|_| ExecutionError::Secret("credential lookup failed".into()))??;
        if secret.is_empty() {
            return Err(ExecutionError::Secret("credential was empty".into()));
        }
        Ok(secret)
    }
}

async fn consume_response(
    response: reqwest::Response,
    target: ResponseTarget<'_>,
) -> Result<Vec<u8>, ExecutionError> {
    let limit = match target {
        ResponseTarget::File(_) => MAX_WEBHOOK_FILE_BYTES,
        ResponseTarget::Capture | ResponseTarget::Discard => MAX_CAPTURED_OUTPUT_BYTES,
    };
    consume_response_with_limit(response, target, limit).await
}

async fn consume_response_with_limit(
    mut response: reqwest::Response,
    target: ResponseTarget<'_>,
    limit: u64,
) -> Result<Vec<u8>, ExecutionError> {
    let stream = match target {
        ResponseTarget::File(_) => "webhook file response",
        ResponseTarget::Capture | ResponseTarget::Discard => "webhook response",
    };
    if response
        .content_length()
        .is_some_and(|length| length > limit)
    {
        return Err(ExecutionError::OutputLimit { stream, limit });
    }

    let mut output = Vec::new();
    let mut file = match target {
        ResponseTarget::File(path) => Some(File::create(path).await?),
        ResponseTarget::Capture | ResponseTarget::Discard => None,
    };
    let mut received = 0_u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| ExecutionError::Webhook("could not read webhook response".into()))?
    {
        let chunk_length = u64::try_from(chunk.len()).unwrap_or(u64::MAX);
        if chunk_length > limit.saturating_sub(received) {
            return Err(ExecutionError::OutputLimit { stream, limit });
        }
        received += chunk_length;

        match target {
            ResponseTarget::Capture => output.extend_from_slice(&chunk),
            ResponseTarget::Discard => {}
            ResponseTarget::File(_) => {
                file.as_mut()
                    .expect("file target has a response destination")
                    .write_all(&chunk)
                    .await?;
            }
        }
    }

    if let Some(file) = file.as_mut() {
        file.flush().await?;
    }
    Ok(output)
}

async fn file_part(path: &Path) -> Result<Part, ExecutionError> {
    let metadata = tokio::fs::metadata(path).await?;
    if !metadata.is_file() {
        return Err(ExecutionError::InvalidConfig(
            "webhook inputs must be regular files".into(),
        ));
    }

    let name = path
        .file_name()
        .ok_or_else(|| ExecutionError::InvalidConfig("webhook input has no filename".into()))?;
    let file = File::open(path).await?;
    let length = file.metadata().await?.len();
    let body = reqwest::Body::wrap_stream(ReaderStream::new(file));
    Ok(Part::stream_with_length(body, length).file_name(name.to_string_lossy().into_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{OutputConfig, OutputMode};
    use crate::staging::StagedFile;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn response_reader_rejects_content_length_before_reading_body() {
        let response =
            mock_response(b"HTTP/1.1 200 OK\r\nContent-Length: 9\r\nConnection: close\r\n\r\n")
                .await;

        let result = consume_response_with_limit(response, ResponseTarget::Capture, 8).await;

        assert_output_limit(result, "webhook response", 8);
    }

    #[tokio::test]
    async fn response_reader_bounds_chunked_capture_and_discard() {
        for target in [ResponseTarget::Capture, ResponseTarget::Discard] {
            let response = mock_response(
                b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n5\r\n12345\r\n4\r\n6789\r\n0\r\n\r\n",
            )
            .await;
            let stream_name = match target {
                ResponseTarget::Capture => "webhook response",
                ResponseTarget::Discard => "webhook response",
                ResponseTarget::File(_) => unreachable!(),
            };

            let result = consume_response_with_limit(response, target, 8).await;

            assert_output_limit(result, stream_name, 8);
        }
    }

    #[tokio::test]
    async fn partial_file_response_is_removed_with_its_staging_guard() {
        let temp = tempfile::tempdir().unwrap();
        let input = temp.path().join("input.txt");
        std::fs::write(&input, b"input").unwrap();
        let config = OutputConfig {
            mode: OutputMode::File,
            extension: Some("out".to_owned()),
            suffix: None,
            overwrite_original: false,
        };
        let staged = StagedFile::new(&input, &config).unwrap();
        let staged_path = staged.path().to_path_buf();
        let response = mock_response(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n5\r\n12345\r\n4\r\n6789\r\n0\r\n\r\n",
        )
        .await;

        let result =
            consume_response_with_limit(response, ResponseTarget::File(&staged_path), 8).await;

        assert_output_limit(result, "webhook file response", 8);
        assert!(staged_path.exists());
        drop(staged);
        assert_no_staging_files(temp.path());
    }

    async fn mock_response(response: &'static [u8]) -> reqwest::Response {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/response", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut chunk = [0; 1024];
            while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0, "request ended before headers");
                request.extend_from_slice(&chunk[..count]);
            }
            stream.write_all(response).await.unwrap();
        });
        let response = Client::builder()
            .no_proxy()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await
            .unwrap();
        server.await.unwrap();
        response
    }

    fn assert_output_limit<T>(
        result: Result<T, ExecutionError>,
        expected_stream: &str,
        expected_limit: u64,
    ) {
        match result {
            Err(error) => {
                let message = error.to_string();
                assert!(
                    message.contains("output limit"),
                    "unexpected error: {message}"
                );
                assert!(
                    message.contains(expected_stream),
                    "unexpected error: {message}"
                );
                assert!(
                    message.contains(&expected_limit.to_string()),
                    "unexpected error: {message}"
                );
            }
            Ok(_) => panic!("expected output-limit error"),
        }
    }

    fn assert_no_staging_files(directory: &Path) {
        assert!(std::fs::read_dir(directory).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".bergamot-tmp-")
        }));
    }
}
