use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use httpgenerator_core::{
    GeneratorSettings, NormalizedOpenApiDocument, OutputType, classify_source,
    generate_http_files, load_and_normalize_document_with_options, openapi::OpenApiSource,
    redact_authorization_headers,
};
use httprunner_core::parser::parse_http_content;
use reqwest::blocking::Client;
use tempfile::Builder;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateOutputType {
    OneRequestPerFile,
    OneFile,
    OneFilePerTag,
}

impl Default for GenerateOutputType {
    fn default() -> Self {
        Self::OneRequestPerFile
    }
}

impl From<GenerateOutputType> for OutputType {
    fn from(value: GenerateOutputType) -> Self {
        match value {
            GenerateOutputType::OneRequestPerFile => OutputType::OneRequestPerFile,
            GenerateOutputType::OneFile => OutputType::OneFile,
            GenerateOutputType::OneFilePerTag => OutputType::OneFilePerTag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub spec: String,
    pub output: PathBuf,
    pub output_type: GenerateOutputType,
    pub base_url: Option<String>,
    pub content_type: String,
    pub authorization_header: Option<String>,
    pub load_authorization_header_from_environment: bool,
    pub authorization_header_variable_name: String,
    pub azure_scope: Option<String>,
    pub azure_tenant_id: Option<String>,
    pub custom_headers: Vec<String>,
    pub skip_headers: bool,
    pub skip_validation: bool,
    pub generate_intellij_tests: bool,
    pub timeout_seconds: u64,
    pub dry_run: bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            spec: String::new(),
            output: PathBuf::from("."),
            output_type: GenerateOutputType::default(),
            base_url: None,
            content_type: "application/json".to_string(),
            authorization_header: None,
            load_authorization_header_from_environment: false,
            authorization_header_variable_name: "authorization".to_string(),
            azure_scope: None,
            azure_tenant_id: None,
            custom_headers: Vec::new(),
            skip_headers: false,
            skip_validation: false,
            generate_intellij_tests: false,
            timeout_seconds: 120,
            dry_run: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub filename: String,
    pub destination: PathBuf,
    pub content: String,
    pub preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateResult {
    pub files: Vec<GeneratedFile>,
    pub dry_run: bool,
}

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("OpenAPI source must not be empty")]
    EmptySource,
    #[error("cannot combine --authorization-header with --load-authorization-header-from-environment")]
    AmbiguousAuthorization,
    #[error("environment variable '{0}' is not set")]
    MissingAuthorizationEnvironment(String),
    #[error("Azure scope-based token acquisition is not implemented yet")]
    AzureScopeUnsupported,
    #[error("invalid generator configuration: {0}")]
    InvalidConfiguration(String),
    #[error("failed to load OpenAPI document: {0}")]
    OpenApiLoad(String),
    #[error("generated file '{filename}' could not be parsed by httprunner-core: {message}")]
    ParseBackValidation { filename: String, message: String },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to fetch protected OpenAPI URL '{url}': {message}")]
    RemoteFetch { url: Url, message: String },
}

impl GenerateError {
    pub fn is_usage_error(&self) -> bool {
        matches!(
            self,
            Self::EmptySource
                | Self::AmbiguousAuthorization
                | Self::MissingAuthorizationEnvironment(_)
                | Self::AzureScopeUnsupported
                | Self::InvalidConfiguration(_)
        )
    }
}

pub fn generate(options: &GenerateOptions) -> Result<GenerateResult, GenerateError> {
    if options.spec.trim().is_empty() {
        return Err(GenerateError::EmptySource);
    }
    if options.authorization_header.is_some() && options.load_authorization_header_from_environment {
        return Err(GenerateError::AmbiguousAuthorization);
    }
    if options.azure_scope.is_some() || options.azure_tenant_id.is_some() {
        return Err(GenerateError::AzureScopeUnsupported);
    }

    let fetch_authorization = resolve_fetch_authorization(options)?;
    let document = load_document(options, fetch_authorization.as_deref())?;

    let settings = GeneratorSettings {
        open_api_path: options.spec.clone(),
        authorization_header: literal_generated_authorization(options),
        authorization_header_from_environment_variable: options
            .load_authorization_header_from_environment,
        authorization_header_variable_name: options.authorization_header_variable_name.clone(),
        content_type: options.content_type.clone(),
        base_url: options.base_url.clone(),
        output_type: options.output_type.into(),
        timeout: options.timeout_seconds,
        generate_intellij_tests: options.generate_intellij_tests,
        custom_headers: options.custom_headers.clone(),
        skip_headers: options.skip_headers,
    };

    let generated = generate_http_files(&settings, &document);
    let mut files = Vec::with_capacity(generated.files.len());

    for file in generated.files {
        parse_http_content(&file.content, None).map_err(|error| GenerateError::ParseBackValidation {
            filename: file.filename.clone(),
            message: error.to_string(),
        })?;

        let destination = options.output.join(&file.filename);
        if !options.dry_run {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&destination, &file.content)?;
        }

        files.push(GeneratedFile {
            filename: file.filename,
            destination,
            preview: redact_authorization_headers(&file.content),
            content: file.content,
        });
    }

    Ok(GenerateResult {
        files,
        dry_run: options.dry_run,
    })
}

fn literal_generated_authorization(options: &GenerateOptions) -> Option<String> {
    if options.load_authorization_header_from_environment {
        None
    } else {
        options.authorization_header.clone()
    }
}

fn resolve_fetch_authorization(options: &GenerateOptions) -> Result<Option<String>, GenerateError> {
    if options.load_authorization_header_from_environment {
        let variable_name = options.authorization_header_variable_name.as_str();
        return env::var(variable_name)
            .map(Some)
            .map_err(|_| GenerateError::MissingAuthorizationEnvironment(variable_name.to_string()));
    }

    Ok(options.authorization_header.clone())
}

fn load_document(
    options: &GenerateOptions,
    authorization_header: Option<&str>,
) -> Result<NormalizedOpenApiDocument, GenerateError> {
    let source = classify_source(&options.spec)
        .map_err(|error| GenerateError::OpenApiLoad(error.to_string()))?;

    match source {
        OpenApiSource::Url(url) if authorization_header.is_some() => {
            load_protected_remote_document(options, url, authorization_header.expect("checked"))
        }
        _ => load_and_normalize_document_with_options(&options.spec, options.skip_validation)
            .map_err(|error| GenerateError::OpenApiLoad(error.to_string())),
    }
}

fn load_protected_remote_document(
    options: &GenerateOptions,
    url: Url,
    authorization_header: &str,
) -> Result<NormalizedOpenApiDocument, GenerateError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(options.timeout_seconds))
        .build()
        .map_err(|error| GenerateError::RemoteFetch {
            url: url.clone(),
            message: error.to_string(),
        })?;

    let response = client
        .get(url.clone())
        .header("Authorization", authorization_header)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|error| GenerateError::RemoteFetch {
            url: url.clone(),
            message: error.to_string(),
        })?;

    let body = response.text().map_err(|error| GenerateError::RemoteFetch {
        url: url.clone(),
        message: error.to_string(),
    })?;

    let suffix = Path::new(url.path())
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| format!(".{value}"))
        .unwrap_or_else(|| ".yaml".to_string());
    let file = Builder::new().suffix(&suffix).tempfile()?;
    fs::write(file.path(), body)?;

    load_and_normalize_document_with_options(&file.path().to_string_lossy(), options.skip_validation)
        .map_err(|error| GenerateError::OpenApiLoad(error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::{GenerateOptions, GenerateOutputType, generate};

    fn fixture(name: &str) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/openapi")
            .join(name)
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn dry_run_keeps_files_in_memory() {
        let temp = tempfile::tempdir().unwrap();
        let options = GenerateOptions {
            spec: fixture("petstore.yaml"),
            output: temp.path().join("generated"),
            output_type: GenerateOutputType::OneFile,
            dry_run: true,
            ..GenerateOptions::default()
        };

        let result = generate(&options).unwrap();

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].destination.exists());
        assert!(result.files[0].preview.contains("@baseUrl"));
    }

    #[test]
    fn writes_generated_files() {
        let temp = tempfile::tempdir().unwrap();
        let options = GenerateOptions {
            spec: fixture("petstore.yaml"),
            output: temp.path().join("generated"),
            output_type: GenerateOutputType::OneRequestPerFile,
            ..GenerateOptions::default()
        };

        let result = generate(&options).unwrap();

        assert_eq!(result.files.len(), 3);
        assert!(result.files.iter().all(|file| file.destination.exists()));
        let first = fs::read_to_string(&result.files[0].destination).unwrap();
        assert!(first.contains("https://petstore.example.com/api"));
    }
}
