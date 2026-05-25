use crate::{HttpFileToolsError, Result};

pub use httprunner_core::{HttpRequest, HttpResult};

#[derive(Debug, Clone, Default)]
pub struct RunOptions<'a> {
    pub environment_name: Option<&'a str>,
    pub verbose: bool,
    pub insecure: bool,
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
    httprunner_core::parser::parse_http_content(content, None).map_err(|source| {
        HttpFileToolsError::ParseHttpFile {
            path: "<memory>".to_string(),
            source,
        }
    })
}

pub fn execute_http_request(request: &HttpRequest, options: &RunOptions<'_>) -> Result<HttpResult> {
    httprunner_core::runner::execute_http_request(request, options.verbose, options.insecure)
        .map_err(|source| HttpFileToolsError::ExecuteHttpRequest {
            request_name: request.name.clone(),
            source,
        })
}
