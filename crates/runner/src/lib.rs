use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use httprunner_core::{
    parser::parse_http_file,
    processor::{ProcessorConfig, format_json_if_valid, process_http_files_with_silent},
    report::{generate_html, generate_markdown},
    types::{Header, HttpResult, ProcessorResults, RequestContext},
};
use serde_json::Value;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Markdown,
    Html,
}

#[derive(Debug, Clone)]
pub struct RunOptions {
    pub paths: Vec<PathBuf>,
    pub discover: bool,
    pub verbose: bool,
    pub pretty_json: bool,
    pub delay_ms: u64,
    pub environment: Option<String>,
    pub insecure: bool,
    pub log: Option<PathBuf>,
    pub report: Option<ReportFormat>,
    pub export: Option<PathBuf>,
    pub fail_fast: bool,
    pub no_banner: bool,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            discover: false,
            verbose: false,
            pretty_json: false,
            delay_ms: 0,
            environment: None,
            insecure: false,
            log: None,
            report: None,
            export: None,
            fail_fast: false,
            no_banner: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Success,
    AssertionFailure,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub status: RunStatus,
    pub files: Vec<PathBuf>,
    pub results: ProcessorResults,
    pub report_path: Option<PathBuf>,
    pub log_path: Option<PathBuf>,
    pub exported_files: Vec<PathBuf>,
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("no .http files were selected")]
    NoFiles,
    #[error("failed to parse '{file}': {message}")]
    Parse { file: PathBuf, message: String },
    #[error("runtime failure: {0}")]
    Runtime(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn run(options: &RunOptions) -> Result<RunResult, RunError> {
    let files = collect_files(&options.paths, options.discover)?;
    if files.is_empty() {
        return Err(RunError::NoFiles);
    }

    for file in &files {
        parse_http_file(&file.to_string_lossy(), options.environment.as_deref()).map_err(
            |error| RunError::Parse {
                file: file.clone(),
                message: error.to_string(),
            },
        )?;
    }

    let results = if options.fail_fast {
        execute_fail_fast(&files, options)?
    } else {
        execute_batch(&files, options)?
    };

    let redacted_results = redact_results(&results, options.pretty_json);
    let report_path = if let Some(format) = options.report {
        Some(write_report(&redacted_results, format)?)
    } else {
        None
    };
    let log_path = if let Some(path) = &options.log {
        fs::write(
            path,
            format_console_output(&redacted_results, options.verbose),
        )?;
        Some(path.clone())
    } else {
        None
    };
    let exported_files = if let Some(directory) = &options.export {
        export_results(directory, &redacted_results, options.pretty_json)?
    } else {
        Vec::new()
    };

    let status = if redacted_results.success {
        RunStatus::Success
    } else {
        RunStatus::AssertionFailure
    };

    Ok(RunResult {
        status,
        files,
        results: redacted_results,
        report_path,
        log_path,
        exported_files,
    })
}

pub fn format_console_output(results: &ProcessorResults, verbose: bool) -> String {
    let mut output = String::new();
    let total_success: u32 = results.files.iter().map(|file| file.success_count).sum();
    let total_failed: u32 = results.files.iter().map(|file| file.failed_count).sum();
    let total_skipped: u32 = results.files.iter().map(|file| file.skipped_count).sum();

    output.push_str("Run summary\n");
    output.push_str("===========\n");
    output.push_str(&format!(
        "passed={} failed={} skipped={}\n",
        total_success, total_failed, total_skipped
    ));

    for file in &results.files {
        output.push_str(&format!(
            "- {} (passed={} failed={} skipped={})\n",
            file.filename, file.success_count, file.failed_count, file.skipped_count
        ));
        if verbose {
            for context in &file.result_contexts {
                match &context.result {
                    Some(result) => output.push_str(&format!(
                        "  * {} {} {} [{} ms]\n",
                        context.request.method,
                        context.request.url,
                        result.status_code,
                        result.duration_ms
                    )),
                    None => output.push_str(&format!(
                        "  * {} {} skipped\n",
                        context.request.method, context.request.url
                    )),
                }
            }
        }
    }

    output
}

fn collect_files(paths: &[PathBuf], discover: bool) -> Result<Vec<PathBuf>, RunError> {
    let mut files = Vec::new();
    let inputs = if paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        paths.to_vec()
    };

    for path in inputs {
        if path.is_file() {
            if path.extension().and_then(|value| value.to_str()) == Some("http") {
                files.push(path);
            }
            continue;
        }

        if path.is_dir() {
            if discover {
                for entry in WalkDir::new(&path).into_iter().filter_map(Result::ok) {
                    let entry_path = entry.path();
                    if entry_path.is_file()
                        && entry_path.extension().and_then(|value| value.to_str()) == Some("http")
                    {
                        files.push(entry_path.to_path_buf());
                    }
                }
            }
            continue;
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn execute_batch(files: &[PathBuf], options: &RunOptions) -> Result<ProcessorResults, RunError> {
    let file_strings: Vec<String> = files
        .iter()
        .map(|file| file.to_string_lossy().into_owned())
        .collect();
    let config = ProcessorConfig::new(&file_strings)
        .with_verbose(options.verbose)
        .with_environment(options.environment.as_deref())
        .with_insecure(options.insecure)
        .with_pretty_json(options.pretty_json)
        .with_silent(true)
        .with_delay(options.delay_ms);
    process_http_files_with_silent(&config).map_err(|error| RunError::Runtime(error.to_string()))
}

fn execute_fail_fast(
    files: &[PathBuf],
    options: &RunOptions,
) -> Result<ProcessorResults, RunError> {
    let mut aggregated = ProcessorResults {
        success: true,
        files: Vec::new(),
    };

    for file in files {
        let file_strings = vec![file.to_string_lossy().into_owned()];
        let config = ProcessorConfig::new(&file_strings)
            .with_verbose(options.verbose)
            .with_environment(options.environment.as_deref())
            .with_insecure(options.insecure)
            .with_pretty_json(options.pretty_json)
            .with_silent(true)
            .with_delay(options.delay_ms);
        let result = process_http_files_with_silent(&config)
            .map_err(|error| RunError::Runtime(error.to_string()))?;
        aggregated.files.extend(result.files);
        if !result.success {
            aggregated.success = false;
            break;
        }
    }

    Ok(aggregated)
}

fn redact_results(results: &ProcessorResults, pretty_json: bool) -> ProcessorResults {
    let mut redacted = results.clone();
    for file in &mut redacted.files {
        for context in &mut file.result_contexts {
            for header in &mut context.request.headers {
                if is_sensitive_header(&header.name) {
                    header.value = "<redacted>".to_string();
                }
            }
            if let Some(body) = &mut context.request.body {
                *body = redact_text(body, pretty_json);
            }
            if let Some(result) = &mut context.result {
                redact_http_result(result, pretty_json);
            }
        }
    }
    redacted
}

fn redact_http_result(result: &mut HttpResult, pretty_json: bool) {
    if let Some(headers) = &mut result.response_headers {
        let keys: Vec<String> = headers.keys().cloned().collect();
        for key in keys {
            if is_sensitive_header(&key) {
                headers.insert(key, "<redacted>".to_string());
            }
        }
    }
    if let Some(body) = &mut result.response_body {
        *body = redact_text(body, pretty_json);
    }
    for assertion in &mut result.assertion_results {
        if let Some(actual) = &mut assertion.actual_value {
            *actual = redact_text(actual, pretty_json);
        }
        if let Some(error_message) = &mut assertion.error_message {
            *error_message = redact_text(error_message, false);
        }
    }
    if let Some(error_message) = &mut result.error_message {
        *error_message = redact_text(error_message, false);
    }
}

fn redact_text(text: &str, pretty_json: bool) -> String {
    let mut value = text.replace("Authorization:", "Authorization: <redacted>");
    if pretty_json && looks_like_json(&value) {
        value = format_json_if_valid(&value);
    }
    value
}

fn looks_like_json(input: &str) -> bool {
    serde_json::from_str::<Value>(input).is_ok()
}

fn is_sensitive_header(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered == "authorization"
        || lowered == "proxy-authorization"
        || lowered.contains("token")
        || lowered.contains("secret")
        || lowered == "x-api-key"
        || lowered == "api-key"
}

fn write_report(results: &ProcessorResults, format: ReportFormat) -> Result<PathBuf, RunError> {
    let filename = match format {
        ReportFormat::Markdown => generate_markdown(results),
        ReportFormat::Html => generate_html(results),
    }
    .map_err(|error| RunError::Runtime(error.to_string()))?;

    Ok(std::env::current_dir()?.join(filename))
}

fn export_results(
    directory: &Path,
    results: &ProcessorResults,
    pretty_json: bool,
) -> Result<Vec<PathBuf>, RunError> {
    fs::create_dir_all(directory)?;
    let mut exported = Vec::new();
    let mut seen = HashSet::new();

    for file in &results.files {
        let stem = Path::new(&file.filename)
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("requests");
        for (index, context) in file.result_contexts.iter().enumerate() {
            let request_base = unique_name(&mut seen, stem, &context.name, index, "request");
            let request_path = directory.join(format!("{request_base}.http"));
            fs::write(&request_path, render_request(context, pretty_json))?;
            exported.push(request_path);

            if context.result.is_some() {
                let response_base = unique_name(&mut seen, stem, &context.name, index, "response");
                let response_path = directory.join(format!("{response_base}.http"));
                fs::write(&response_path, render_response(context, pretty_json))?;
                exported.push(response_path);
            }
        }
    }

    Ok(exported)
}

fn unique_name(
    seen: &mut HashSet<String>,
    stem: &str,
    request_name: &str,
    index: usize,
    suffix: &str,
) -> String {
    let base = format!(
        "{}-{:03}-{}-{}",
        sanitize(stem),
        index,
        sanitize(request_name),
        suffix
    );
    if seen.insert(base.clone()) {
        return base;
    }

    let mut counter = 1usize;
    loop {
        let candidate = format!("{base}-{counter}");
        if seen.insert(candidate.clone()) {
            return candidate;
        }
        counter += 1;
    }
}

fn sanitize(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    sanitized.trim_matches('-').to_string()
}

fn render_request(context: &RequestContext, pretty_json: bool) -> String {
    let mut output = format!("{} {}\n", context.request.method, context.request.url);
    write_headers(&mut output, &context.request.headers);
    output.push('\n');
    if let Some(body) = &context.request.body {
        output.push_str(&render_body(body, pretty_json));
        output.push('\n');
    }
    output
}

fn render_response(context: &RequestContext, pretty_json: bool) -> String {
    let result = context.result.as_ref().expect(
        "result must be Some because context.result.is_some() was verified in export_results",
    );
    let mut output = format!("HTTP/1.1 {}\n", result.status_code);
    if let Some(headers) = &result.response_headers {
        let mut values: Vec<Header> = headers
            .iter()
            .map(|(name, value)| Header {
                name: name.clone(),
                value: value.clone(),
            })
            .collect();
        values.sort_by(|left, right| left.name.cmp(&right.name));
        write_headers(&mut output, &values);
    }
    output.push('\n');
    if let Some(body) = &result.response_body {
        output.push_str(&render_body(body, pretty_json));
        output.push('\n');
    }
    output
}

fn render_body(body: &str, pretty_json: bool) -> String {
    if pretty_json {
        format_json_if_valid(body)
    } else {
        body.to_string()
    }
}

fn write_headers(output: &mut String, headers: &[Header]) {
    for header in headers {
        output.push_str(&format!("{}: {}\n", header.name, header.value));
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use super::{ReportFormat, RunOptions, RunStatus, format_console_output, run};

    fn sample_http(url: &str) -> String {
        format!("GET {url}/health\nAccept: application/json\n\nHTTP 200\n")
    }

    fn start_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buffer = [0_u8; 1024];
                let _ = stream.read(&mut buffer);
                let response = concat!(
                    "HTTP/1.1 200 OK\r\n",
                    "Content-Type: application/json\r\n",
                    "Content-Length: 15\r\n",
                    "Connection: close\r\n",
                    "\r\n",
                    "{\"ok\": true}\r\n"
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        format!("http://{}", address)
    }

    #[test]
    fn run_requires_files() {
        let error = run(&RunOptions::default()).unwrap_err();
        assert!(error.to_string().contains("no .http files"));
    }

    #[test]
    fn discover_finds_http_files() {
        let temp = tempfile::tempdir().unwrap();
        let nested = temp.path().join("nested");
        fs::create_dir_all(&nested).unwrap();
        let url = start_server();
        fs::write(nested.join("sample.http"), sample_http(&url)).unwrap();

        let result = run(&RunOptions {
            paths: vec![temp.path().to_path_buf()],
            discover: true,
            ..RunOptions::default()
        })
        .unwrap();

        assert_eq!(result.files.len(), 1);
    }

    #[test]
    fn writes_log_and_report() {
        let temp = tempfile::tempdir().unwrap();
        let log_path = temp.path().join("run.log");
        let http_path = temp.path().join("sample.http");
        let url = start_server();
        fs::write(&http_path, sample_http(&url)).unwrap();

        let result = run(&RunOptions {
            paths: vec![http_path],
            log: Some(log_path.clone()),
            report: Some(ReportFormat::Markdown),
            ..RunOptions::default()
        })
        .unwrap();

        assert_eq!(result.status, RunStatus::Success);
        assert!(log_path.exists());
        let report_path = result.report_path.unwrap();
        assert!(report_path.exists());
        assert!(format_console_output(&result.results, false).contains("Run summary"));
        fs::remove_file(report_path).unwrap();
    }
}
