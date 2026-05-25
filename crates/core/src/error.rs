use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, HttpFileToolsError>;

#[derive(Debug)]
pub enum HttpFileToolsError {
    Generate(httpgenerator_core::openapi::OpenApiDocumentNormalizationError),
    ParseHttpFile {
        path: String,
        source: anyhow::Error,
    },
    ExecuteHttpRequest {
        request_name: Option<String>,
        source: anyhow::Error,
    },
}

impl fmt::Display for HttpFileToolsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generate(source) => write!(f, "failed to generate HTTP files: {source}"),
            Self::ParseHttpFile { path, source } => {
                write!(f, "failed to parse HTTP file '{path}': {source}")
            }
            Self::ExecuteHttpRequest {
                request_name,
                source,
            } => match request_name {
                Some(name) => write!(f, "failed to execute HTTP request '{name}': {source}"),
                None => write!(f, "failed to execute HTTP request: {source}"),
            },
        }
    }
}

impl Error for HttpFileToolsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Generate(source) => Some(source),
            Self::ParseHttpFile { source, .. } | Self::ExecuteHttpRequest { source, .. } => {
                Some(source.as_ref())
            }
        }
    }
}
