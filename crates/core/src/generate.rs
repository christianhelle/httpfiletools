use crate::{HttpFileToolsError, Result};

pub use httpgenerator_core::NormalizedOpenApiDocument;
pub use httpgenerator_core::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};

pub fn generate(settings: &GeneratorSettings) -> Result<GeneratorResult> {
    let document =
        httpgenerator_core::openapi::load_and_normalize_document(&settings.open_api_path)
            .map_err(HttpFileToolsError::Generate)?;

    Ok(generate_from_document(settings, &document))
}

pub fn generate_from_document(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
) -> GeneratorResult {
    httpgenerator_core::generate_http_files(settings, document)
}
