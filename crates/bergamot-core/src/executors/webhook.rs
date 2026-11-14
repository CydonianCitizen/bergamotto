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
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

use crate::descriptor::ExecutorConfig;

use super::{ExecutionError, ExecutionOutput};

const SECRET_PLACEHOLDER: &str = "${secret}";

/// Retrieves secrets referenced by webhook action manifests.
pub trait SecretProvider: Send + Sync {
    fn get_secret(&self, key: &str) -> Result<String, ExecutionError>;
}

/// Reads credentials stored under the `bergamot` service in the OS credential store.
#[derive(Debug, Default)]
pub struct KeyringSecretProvider;

impl SecretProvider for KeyringSecretProvider {
    fn get_secret(&self, key: &str) -> Result<String, ExecutionError> {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        {
            let entry = keyring::Entry::new("bergamot", key).map_err(|_| {
                ExecutionError::Secret("could not access OS credential store".into())
            })?;
            entry
                .get_password()
                .map_err(|_| ExecutionError::Secret("credential was not available".into()))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            let _ = key;
            Err(ExecutionError::Secret(
                "no OS credential store for this platform".into(),
            ))
        }
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
            method,
            url,
            headers,
            file_field_name,
            files,
            credential_key.as_deref(),
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
        method: Method,
        url: Url,
        headers: &BTreeMap<String, String>,
        file_field_name: &str,
        files: &[PathBuf],
        credential_key: Option<&str>,
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

        let mut request = self.client.request(method, url);
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

        let stdout = response
            .bytes()
            .await
            .map_err(|_| ExecutionError::Webhook("could not read webhook response".into()))?
            .to_vec();

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
            .map_err(|_| ExecutionError::Secret("credential lookup failed".into()))?
            .map_err(|_| ExecutionError::Secret("credential lookup failed".into()))?;
        if secret.is_empty() {
            return Err(ExecutionError::Secret("credential was empty".into()));
        }
        Ok(secret)
    }
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
