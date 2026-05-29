use crate::{HttpFileToolsError, Result};

pub use httprunner_core::{
    HttpRequest, HttpResult,
    processor::{ProcessorConfig, RequestProcessingResult},
    types::{HttpFileResults, ProcessorResults, RequestContext},
};

#[derive(Debug, Clone, Default)]
pub struct RunRequest {
    pub files: Vec<String>,
    pub options: RunOptions,
}

impl RunRequest {
    pub fn new(files: impl Into<Vec<String>>) -> Self {
        Self {
            files: files.into(),
            options: RunOptions::default(),
        }
    }

    pub fn from_file(file: impl Into<String>) -> Self {
        Self::new(vec![file.into()])
    }

    pub fn with_options(files: impl Into<Vec<String>>, options: RunOptions) -> Self {
        Self {
            files: files.into(),
            options,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RunOptions {
    pub environment_name: Option<String>,
    pub verbose: bool,
    pub log_filename: Option<String>,
    pub insecure: bool,
    pub pretty_json: bool,
    pub silent: bool,
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Markdown,
    Html,
}

pub fn run_http_files(request: &RunRequest) -> Result<ProcessorResults> {
    let config = ProcessorConfig::new(&request.files)
        .with_verbose(request.options.verbose)
        .with_log_filename(request.options.log_filename.as_deref())
        .with_environment(request.options.environment_name.as_deref())
        .with_insecure(request.options.insecure)
        .with_pretty_json(request.options.pretty_json)
        .with_silent(request.options.silent)
        .with_delay(request.options.delay_ms);

    httprunner_core::processor::process_http_files_with_silent(&config).map_err(|source| {
        HttpFileToolsError::Run {
            files: request.files.clone(),
            source,
        }
    })
}

pub fn run(request: &RunRequest) -> Result<ProcessorResults> {
    run_http_files(request)
}

pub fn discover_http_files() -> Result<Vec<String>> {
    httprunner_core::discovery::run_discovery_mode().map_err(|source| HttpFileToolsError::Run {
        files: Vec::new(),
        source,
    })
}

pub fn export_results(results: &ProcessorResults, pretty_json: bool) -> Result<Vec<String>> {
    let export =
        httprunner_core::export::export_results(results, pretty_json).map_err(|source| {
            HttpFileToolsError::Export {
                operation: "export results",
                source,
            }
        })?;

    if export.failed_file_names.is_empty() {
        Ok(export.file_names)
    } else {
        Err(HttpFileToolsError::ExportFailures {
            failures: export.failed_file_names,
        })
    }
}

pub fn export_json(results: &ProcessorResults) -> Result<String> {
    httprunner_core::export::export_json(results).map_err(|source| HttpFileToolsError::Export {
        operation: "export JSON results",
        source,
    })
}

pub fn generate_report(results: &ProcessorResults, format: ReportFormat) -> Result<String> {
    match format {
        ReportFormat::Markdown => httprunner_core::report::generate_markdown(results),
        ReportFormat::Html => httprunner_core::report::generate_html(results),
    }
    .map_err(|source| HttpFileToolsError::Export {
        operation: "generate report",
        source,
    })
}

pub fn parse_http_file(path: &str, environment_name: Option<&str>) -> Result<Vec<HttpRequest>> {
    httprunner_core::parser::parse_http_file(path, environment_name).map_err(|source| {
        HttpFileToolsError::ParseHttpFile {
            path: path.to_string(),
            source,
        }
    })
}

pub fn parse_http_content(content: &str) -> Result<Vec<HttpRequest>> {
    httprunner_core::parser::parse_http_content(content, None)
        .map_err(|source| HttpFileToolsError::ParseHttpContent { source })
}

pub fn execute_http_request(request: &HttpRequest, options: &RunOptions) -> Result<HttpResult> {
    httprunner_core::runner::execute_http_request(request, options.verbose, options.insecure)
        .map_err(|source| HttpFileToolsError::ExecuteHttpRequest {
            request_name: request.name.clone(),
            source,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn run_request_accepts_single_file() {
        let request = RunRequest::from_file("sample.http");

        assert_eq!(request.files, vec!["sample.http"]);
        assert_eq!(request.options, RunOptions::default());
    }

    #[test]
    fn run_http_files_delegates_to_upstream_processor() {
        let request = RunRequest::with_options(
            Vec::<String>::new(),
            RunOptions {
                silent: true,
                ..RunOptions::default()
            },
        );

        let result = run_http_files(&request).expect("empty run succeeds upstream");

        assert!(result.success);
        assert!(result.files.is_empty());
    }

    #[test]
    fn parse_http_content_uses_upstream_parser() {
        let requests = parse_http_content("GET https://example.test").expect("request parses");

        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "GET");
        assert_eq!(requests[0].url, "https://example.test");
    }

    #[test]
    fn parse_http_file_error_preserves_source_diagnostics() {
        let error =
            parse_http_file("does-not-exist.http", None).expect_err("missing file should fail");

        assert!(matches!(error, HttpFileToolsError::ParseHttpFile { .. }));
        assert!(error.source().is_some());
        assert!(
            error
                .to_string()
                .contains("failed to parse HTTP file 'does-not-exist.http'")
        );
    }
}
