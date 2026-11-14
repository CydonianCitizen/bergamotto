use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use bergamot_core::executors::webhook::{SecretProvider, WebhookExecutor};
use bergamot_core::{CancellationToken, ExecutionError, ExecutorConfig};
use tempfile::tempdir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

struct TestSecret;

impl SecretProvider for TestSecret {
    fn get_secret(&self, key: &str) -> Result<String, ExecutionError> {
        assert_eq!(key, "test-credential");
        Ok("local-test-token".to_owned())
    }
}

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
        .execute(&config(url, BTreeMap::new(), 5), &files, &CancellationToken::new())
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
        .execute(&config(url, BTreeMap::new(), 1), &[file], &CancellationToken::new())
        .await
        .unwrap_err();
    assert!(matches!(error, ExecutionError::TimedOut(1)));
    partial_received.try_recv().unwrap();
    server.abort();
}

fn test_executor() -> WebhookExecutor {
    WebhookExecutor::with_client_builder(
        Arc::new(TestSecret),
        reqwest::Client::builder().no_proxy(),
    )
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
