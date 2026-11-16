use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use bergamot_core::executors::webhook::WebhookExecutor;
use bergamot_core::{
    ActionInvocation, ActionRegistry, CancellationToken, Dispatcher, DragContext, ExecutionError,
    ExecutorConfig, MemorySecretStore, OutputMode, Platform, SecretStore,
};
use tempfile::tempdir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

const VALID_PROCESS: &str = include_str!("../../../shared/fixtures/valid-process.json");

#[tokio::test]
async fn webhook_posts_multiple_files_with_bearer_credentials() {
    let files = [PathBuf::from("first file.txt"), PathBuf::from("second.txt")];
    let temp = tempdir().unwrap();
    let files = files.map(|name| temp.path().join(name));
    std::fs::write(&files[0], b"first contents").unwrap();
    std::fs::write(&files[1], b"second contents").unwrap();
    let (listener, url) = mock_listener().await;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let (headers, body) = read_request(&mut stream).await;
        assert!(headers.starts_with("post /upload http/1.1\r\n"));
        assert!(headers.contains("authorization: bearer local-test-token\r\n"));
        assert!(headers.contains("content-type: multipart/form-data; boundary="));
        assert_eq!(body.matches("name=\"attachment\"").count(), 2);
        assert!(body.contains("filename=\"first file.txt\""));
        assert!(body.contains("filename=\"second.txt\""));
        assert!(body.contains("first contents"));
        assert!(body.contains("second contents"));
        send_response(&mut stream, "200 OK", "received").await;
    });

    let executor = test_executor();
    let output = executor
        .execute(
            &config(url, BTreeMap::new(), 5),
            &files,
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    assert_eq!(output.stdout, b"received");
    server.await.unwrap();
}

#[tokio::test]
async fn webhook_substitutes_a_header_secret_and_rejects_failed_status() {
    let temp = tempdir().unwrap();
    let file = temp.path().join("input.txt");
    std::fs::write(&file, b"input").unwrap();
    let (listener, url) = mock_listener().await;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let (headers, _) = read_request(&mut stream).await;
        assert!(headers.contains("x-api-token: token local-test-token\r\n"));
        assert!(!headers.contains("authorization:"));
        send_response(&mut stream, "403 Forbidden", "request denied").await;
    });

    let executor = test_executor();
    let headers = BTreeMap::from([("X-Api-Token".to_owned(), "Token ${secret}".to_owned())]);
    let error = executor
        .execute(&config(url, headers, 5), &[file], &CancellationToken::new())
        .await
        .unwrap_err();
    assert!(matches!(error, ExecutionError::Webhook(message) if message.contains("403")));
    server.await.unwrap();
}

#[tokio::test]
async fn webhook_times_out_while_reading_a_response() {
    let temp = tempdir().unwrap();
    let file = temp.path().join("input.txt");
    std::fs::write(&file, b"input").unwrap();
    let (listener, url) = mock_listener().await;
    let (partial_sent, mut partial_received) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\npartial")
            .await
            .unwrap();
        partial_sent.send(()).unwrap();
        tokio::time::sleep(Duration::from_secs(10)).await;
    });

    let executor = test_executor();
    let error = executor
        .execute(
            &config(url, BTreeMap::new(), 1),
            &[file],
            &CancellationToken::new(),
        )
        .await
        .unwrap_err();
    assert!(matches!(error, ExecutionError::TimedOut(1)));
    partial_received.try_recv().unwrap();
    server.abort();
}

#[tokio::test]
async fn webhook_file_response_streams_to_staging_and_commits() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    std::fs::write(&input, b"input").unwrap();
    let (listener, url) = mock_listener().await;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        send_chunked_response(&mut stream, &[b"streamed ", b"file output"]).await;
    });
    let (dispatcher, invocation) =
        setup_dispatcher(&temp.path().join("actions"), url, OutputMode::File);
    let (progress, _events) = mpsc::channel(4);

    let result = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await
        .unwrap();

    let expected = temp.path().join("input.out");
    assert_eq!(result.output_files, std::slice::from_ref(&expected));
    assert_eq!(std::fs::read(&expected).unwrap(), b"streamed file output");
    assert_no_staging_files(temp.path());
    server.await.unwrap();
}

#[tokio::test]
async fn webhook_none_mode_checks_content_length_limit() {
    const CAPTURE_LIMIT: u64 = 8 * 1024 * 1024;
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    std::fs::write(&input, b"input").unwrap();
    let (listener, url) = mock_listener().await;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            CAPTURE_LIMIT + 1,
        );
        stream.write_all(header.as_bytes()).await.unwrap();
    });
    let (dispatcher, invocation) =
        setup_dispatcher(&temp.path().join("actions"), url, OutputMode::None);
    let (progress, _events) = mpsc::channel(4);

    let result = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await;

    assert_output_limit(result, "webhook response", CAPTURE_LIMIT);
    server.await.unwrap();
}

#[tokio::test]
async fn oversized_webhook_file_response_drops_partial_staging() {
    const FILE_LIMIT: u64 = 1024 * 1024 * 1024;
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    std::fs::write(&input, b"original").unwrap();
    let (listener, url) = mock_listener().await;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            FILE_LIMIT + 1,
        );
        stream.write_all(header.as_bytes()).await.unwrap();
    });
    let (dispatcher, invocation) =
        setup_dispatcher(&temp.path().join("actions"), url, OutputMode::File);
    let (progress, _events) = mpsc::channel(4);

    let result = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await;

    assert_output_limit(result, "webhook file response", FILE_LIMIT);
    assert_eq!(std::fs::read(&input).unwrap(), b"original");
    assert_no_staging_files(temp.path());
    assert!(!temp.path().join("input.out").exists());
    server.await.unwrap();
}

fn test_executor() -> WebhookExecutor {
    let store = MemorySecretStore::default();
    store
        .set_secret("test-credential", "local-test-token")
        .unwrap();
    WebhookExecutor::with_client_builder(Arc::new(store), reqwest::Client::builder().no_proxy())
        .unwrap()
}

fn config(url: String, headers: BTreeMap<String, String>, timeout_seconds: u32) -> ExecutorConfig {
    ExecutorConfig::Webhook {
        method: "POST".to_owned(),
        url,
        headers,
        file_field_name: "attachment".to_owned(),
        credential_key: Some("test-credential".to_owned()),
        timeout_seconds,
    }
}

fn setup_dispatcher(
    actions_directory: &Path,
    url: String,
    output_mode: OutputMode,
) -> (Dispatcher, ActionInvocation) {
    let registry = Arc::new(ActionRegistry::open(actions_directory).unwrap());
    let mut action = ActionRegistry::parse_manifest(VALID_PROCESS).unwrap();
    action.output.mode = output_mode;
    action.output.extension = Some("out".to_owned());
    action.executor = ExecutorConfig::Webhook {
        method: "POST".to_owned(),
        url,
        headers: BTreeMap::new(),
        file_field_name: "attachment".to_owned(),
        credential_key: None,
        timeout_seconds: 5,
    };
    registry.save_action(&action).unwrap();
    let dispatcher = Dispatcher::with_secret_provider(
        Arc::clone(&registry),
        Arc::new(MemorySecretStore::default()),
    )
    .unwrap();
    let invocation = ActionInvocation {
        action_id: action.id,
        context: DragContext {
            platform: Platform::current().unwrap(),
            files: vec![actions_directory.parent().unwrap().join("input.txt")],
        },
    };
    (dispatcher, invocation)
}

async fn send_chunked_response(stream: &mut TcpStream, chunks: &[&[u8]]) {
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    for chunk in chunks {
        stream
            .write_all(format!("{:X}\r\n", chunk.len()).as_bytes())
            .await
            .unwrap();
        stream.write_all(chunk).await.unwrap();
        stream.write_all(b"\r\n").await.unwrap();
    }
    stream.write_all(b"0\r\n\r\n").await.unwrap();
}

fn assert_output_limit<T>(result: Result<T, ExecutionError>, stream: &str, limit: u64) {
    match result {
        Err(error) => {
            let message = error.to_string();
            assert!(
                message.contains("output limit"),
                "unexpected error: {message}"
            );
            assert!(message.contains(stream), "unexpected error: {message}");
            assert!(
                message.contains(&limit.to_string()),
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

async fn mock_listener() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/upload", listener.local_addr().unwrap());
    (listener, url)
}

async fn read_request(stream: &mut TcpStream) -> (String, String) {
    let mut buffer = Vec::new();
    let mut chunk = [0; 4096];
    let header_end = loop {
        let count = stream.read(&mut chunk).await.unwrap();
        assert!(count > 0, "request ended before headers");
        buffer.extend_from_slice(&chunk[..count]);
        if let Some(index) = buffer.windows(4).position(|value| value == b"\r\n\r\n") {
            break index + 4;
        }
    };
    let headers = String::from_utf8_lossy(&buffer[..header_end]).to_ascii_lowercase();
    let body_length: usize = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .expect("known-length multipart body")
        .parse()
        .unwrap();
    while buffer.len() < header_end + body_length {
        let count = stream.read(&mut chunk).await.unwrap();
        assert!(count > 0, "request ended before body");
        buffer.extend_from_slice(&chunk[..count]);
    }
    let body = String::from_utf8(buffer[header_end..header_end + body_length].to_vec()).unwrap();
    (headers, body)
}

async fn send_response(stream: &mut TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    stream.write_all(response.as_bytes()).await.unwrap();
}
