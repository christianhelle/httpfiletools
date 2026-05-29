use std::{
    error::Error,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    sync::mpsc,
    thread,
    time::Duration,
};

use httpfiletools_core::{
    GenerateOptions, GenerateRequest, OutputType, RunOptions, execute_http_request,
    generate_http_files, parse_http_content,
};

fn fixture_path(relative: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(relative)
        .to_string_lossy()
        .into_owned()
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

#[test]
fn generate_one_file_matches_golden_http_fixture() -> Result<(), Box<dyn Error>> {
    let request = GenerateRequest::with_options(
        fixture_path("openapi/compat-openapi.yaml"),
        GenerateOptions {
            output_type: OutputType::OneFile,
            ..GenerateOptions::default()
        },
    );

    let result = generate_http_files(&request)?;

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].filename, "Requests.http");
    assert_eq!(
        normalize_newlines(&result.files[0].content),
        include_str!("fixtures/golden/generate-one-file.http")
    );

    Ok(())
}

#[test]
fn invalid_openapi_fixture_reports_stable_generate_error_prefix() {
    let request = GenerateRequest::new(fixture_path("openapi/invalid-missing-path-parameter.yaml"));

    let error = generate_http_files(&request).expect_err("invalid OpenAPI should fail");
    let message = error.to_string();

    assert!(message.starts_with("failed to generate HTTP files from '"));
    assert!(message.contains("invalid-missing-path-parameter.yaml"));
}

#[test]
fn local_http_fixture_executes_against_deterministic_server() -> Result<(), Box<dyn Error>> {
    let (port, received_request, server) = spawn_health_server()?;
    let http = include_str!("fixtures/http/local-health.http").replace("{{port}}", &port);
    let requests = parse_http_content(&http)?;

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].url, format!("http://127.0.0.1:{port}/health"));

    let result = execute_http_request(
        &requests[0],
        &RunOptions {
            verbose: true,
            ..RunOptions::default()
        },
    )?;

    server.join().expect("server thread should finish");
    let raw_request = received_request.recv_timeout(Duration::from_secs(1))?;

    assert!(raw_request.starts_with("get /health http/1.1"));
    assert!(raw_request.contains("accept: application/json"));
    assert_eq!(result.status_code, 200);
    assert!(result.success);
    assert_eq!(result.response_body.as_deref(), Some("{\"status\":\"ok\"}"));

    Ok(())
}

fn spawn_health_server()
-> Result<(String, mpsc::Receiver<String>, thread::JoinHandle<()>), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port().to_string();
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("server accepts one request");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("server read timeout is set");

        let mut buffer = [0_u8; 4096];
        let bytes_read = stream.read(&mut buffer).expect("server reads request");
        let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        tx.send(raw_request.to_ascii_lowercase())
            .expect("request is sent to test thread");

        let body = "{\"status\":\"ok\"}";
        write!(
            stream,
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .expect("server writes response");
    });

    Ok((port, rx, handle))
}
