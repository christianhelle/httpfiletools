use crate::{HttpFileToolsError, Result};

pub use httpgenerator_core::NormalizedOpenApiDocument;
pub use httpgenerator_core::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateRequest {
    pub open_api_path: String,
    pub options: GenerateOptions,
}

impl GenerateRequest {
    pub fn new(open_api_path: impl Into<String>) -> Self {
        Self {
            open_api_path: open_api_path.into(),
            options: GenerateOptions::default(),
        }
    }

    pub fn with_options(open_api_path: impl Into<String>, options: GenerateOptions) -> Self {
        Self {
            open_api_path: open_api_path.into(),
            options,
        }
    }

    pub fn to_generator_settings(&self) -> GeneratorSettings {
        let defaults = GeneratorSettings::default();

        GeneratorSettings {
            open_api_path: self.open_api_path.clone(),
            authorization_header: self.options.authorization_header.clone(),
            authorization_header_from_environment_variable: self
                .options
                .authorization_header_from_environment_variable,
            authorization_header_variable_name: self
                .options
                .authorization_header_variable_name
                .clone()
                .unwrap_or(defaults.authorization_header_variable_name),
            content_type: self
                .options
                .content_type
                .clone()
                .unwrap_or(defaults.content_type),
            base_url: self.options.base_url.clone(),
            output_type: self.options.output_type,
            timeout: self.options.timeout.unwrap_or(defaults.timeout),
            generate_intellij_tests: self.options.generate_intellij_tests,
            custom_headers: self.options.custom_headers.clone(),
            skip_headers: self.options.skip_headers,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateOptions {
    pub authorization_header: Option<String>,
    pub authorization_header_from_environment_variable: bool,
    pub authorization_header_variable_name: Option<String>,
    pub content_type: Option<String>,
    pub base_url: Option<String>,
    pub output_type: OutputType,
    pub timeout: Option<u64>,
    pub generate_intellij_tests: bool,
    pub custom_headers: Vec<String>,
    pub skip_headers: bool,
    pub tolerate_invalid_openapi31: bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: None,
            content_type: None,
            base_url: None,
            output_type: OutputType::default(),
            timeout: None,
            generate_intellij_tests: false,
            custom_headers: Vec::new(),
            skip_headers: false,
            tolerate_invalid_openapi31: false,
        }
    }
}

pub fn generate_http_files(request: &GenerateRequest) -> Result<GeneratorResult> {
    let settings = request.to_generator_settings();
    let document = httpgenerator_core::openapi::load_and_normalize_document_with_options(
        &settings.open_api_path,
        request.options.tolerate_invalid_openapi31,
    )
    .map_err(|source| HttpFileToolsError::Generate {
        open_api_path: settings.open_api_path.clone(),
        source,
    })?;

    Ok(generate_from_document(&settings, &document))
}

pub fn generate(settings: &GeneratorSettings) -> Result<GeneratorResult> {
    let request = GenerateRequest::with_options(
        settings.open_api_path.clone(),
        GenerateOptions {
            authorization_header: settings.authorization_header.clone(),
            authorization_header_from_environment_variable: settings
                .authorization_header_from_environment_variable,
            authorization_header_variable_name: Some(
                settings.authorization_header_variable_name.clone(),
            ),
            content_type: Some(settings.content_type.clone()),
            base_url: settings.base_url.clone(),
            output_type: settings.output_type,
            timeout: Some(settings.timeout),
            generate_intellij_tests: settings.generate_intellij_tests,
            custom_headers: settings.custom_headers.clone(),
            skip_headers: settings.skip_headers,
            tolerate_invalid_openapi31: false,
        },
    );

    generate_http_files(&request)
}

pub fn generate_from_document(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
) -> GeneratorResult {
    httpgenerator_core::generate_http_files(settings, document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn generate_request_maps_to_upstream_settings() {
        let request = GenerateRequest::with_options(
            "openapi.yaml",
            GenerateOptions {
                authorization_header: Some("Bearer token".to_string()),
                authorization_header_from_environment_variable: true,
                authorization_header_variable_name: Some("AUTH".to_string()),
                content_type: Some("application/problem+json".to_string()),
                base_url: Some("https://example.test".to_string()),
                output_type: OutputType::OneFile,
                timeout: Some(30),
                generate_intellij_tests: true,
                custom_headers: vec!["X-Test: true".to_string()],
                skip_headers: true,
                tolerate_invalid_openapi31: true,
            },
        );

        let settings = request.to_generator_settings();

        assert_eq!(settings.open_api_path, "openapi.yaml");
        assert_eq!(
            settings.authorization_header,
            Some("Bearer token".to_string())
        );
        assert!(settings.authorization_header_from_environment_variable);
        assert_eq!(settings.authorization_header_variable_name, "AUTH");
        assert_eq!(settings.content_type, "application/problem+json");
        assert_eq!(settings.base_url, Some("https://example.test".to_string()));
        assert_eq!(settings.output_type, OutputType::OneFile);
        assert_eq!(settings.timeout, 30);
        assert!(settings.generate_intellij_tests);
        assert_eq!(settings.custom_headers, vec!["X-Test: true"]);
        assert!(settings.skip_headers);
    }

    #[test]
    fn generate_error_preserves_source_diagnostics() {
        let request = GenerateRequest::new("");
        let error = generate_http_files(&request).expect_err("empty source should fail");

        assert!(matches!(error, HttpFileToolsError::Generate { .. }));
        assert!(error.source().is_some());
        assert!(
            error
                .to_string()
                .contains("failed to generate HTTP files from ''")
        );
    }
}
