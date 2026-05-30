use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use httpfiletools_core::{
    GenerateOptions, GenerateRequest, OutputType, ReportFormat, RunOptions, RunRequest,
    discover_http_files, export_json, export_results, generate_http_files, generate_report,
    run_http_files,
};

#[derive(Debug, Parser)]
#[command(
    name = "httpfiletools",
    version,
    about = "Tools for generating and running .http files",
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        about = "Generate .http files from OpenAPI specifications",
        disable_version_flag = true
    )]
    Generate(GenerateArgs),
    #[command(about = "Run .http files", disable_version_flag = true)]
    Run(RunArgs),
}

#[derive(Debug, Parser)]
struct GenerateArgs {
    #[arg(
        value_name = "OPENAPI",
        help = "URL or file path to OpenAPI Specification file"
    )]
    openapi: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        default_value = "./"
    )]
    output: PathBuf,

    #[arg(short = 'v', long = "version", action = clap::ArgAction::SetTrue)]
    version: bool,

    #[arg(long = "no-logging", action = clap::ArgAction::SetTrue)]
    no_logging: bool,

    #[arg(long = "skip-validation", action = clap::ArgAction::SetTrue)]
    skip_validation: bool,

    #[arg(long = "authorization-header", value_name = "HEADER")]
    authorization_header: Option<String>,

    #[arg(long = "load-authorization-header-from-environment", action = clap::ArgAction::SetTrue)]
    load_authorization_header_from_environment: bool,

    #[arg(
        long = "authorization-header-variable-name",
        value_name = "VARIABLE-NAME",
        default_value = "authorization"
    )]
    authorization_header_variable_name: String,

    #[arg(
        long = "content-type",
        value_name = "CONTENT-TYPE",
        default_value = "application/json"
    )]
    content_type: String,

    #[arg(long = "base-url", value_name = "BASE-URL")]
    base_url: Option<String>,

    #[arg(long = "output-type", value_name = "OUTPUT-TYPE", default_value_t = OutputTypeArg::OneRequestPerFile, ignore_case = true)]
    output_type: OutputTypeArg,

    #[arg(long = "azure-scope", value_name = "SCOPE")]
    azure_scope: Option<String>,

    #[arg(long = "azure-tenant-id", value_name = "TENANT-ID")]
    azure_tenant_id: Option<String>,

    #[arg(long = "timeout", value_name = "SECONDS", default_value_t = 120)]
    timeout: u64,

    #[arg(long = "generate-intellij-tests", action = clap::ArgAction::SetTrue)]
    generate_intellij_tests: bool,

    #[arg(long = "custom-header", value_name = "HEADER")]
    custom_headers: Vec<String>,

    #[arg(long = "skip-headers", action = clap::ArgAction::SetTrue)]
    skip_headers: bool,
}

#[derive(Debug, Parser)]
struct RunArgs {
    #[arg(
        value_name = "FILE",
        conflicts_with = "discover",
        help = "One or more .http files to process"
    )]
    files: Vec<String>,

    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::SetTrue, help = "Show detailed HTTP request and response information")]
    verbose: bool,

    #[arg(long = "version", action = clap::ArgAction::SetTrue, help = "Print version information")]
    version: bool,

    #[arg(long = "log", value_name = "FILENAME", num_args = 0..=1, default_missing_value = "log", help = "Log output to a file (defaults to 'log' if no filename is specified)")]
    log: Option<String>,

    #[arg(
        long = "env",
        value_name = "ENVIRONMENT",
        help = "Specify environment name to load variables from http-client.env.json"
    )]
    environment: Option<String>,

    #[arg(long = "insecure", action = clap::ArgAction::SetTrue, help = "Allow insecure HTTPS connections (accept invalid certificates and hostnames)")]
    insecure: bool,

    #[arg(long = "discover", action = clap::ArgAction::SetTrue, help = "Recursively discover and process all .http files from current directory")]
    discover: bool,

    #[arg(long = "no-banner", action = clap::ArgAction::SetTrue, help = "Do not show the donation banner")]
    no_banner: bool,

    #[arg(long = "pretty-json", action = clap::ArgAction::SetTrue, help = "Pretty-print JSON payloads in verbose output")]
    pretty_json: bool,

    #[arg(long = "report", value_name = "FORMAT", num_args = 0..=1, default_missing_value = "markdown", ignore_case = true, help = "Generate summary report (default=markdown)")]
    report: Option<ReportFormatArg>,

    #[arg(long = "export", action = clap::ArgAction::SetTrue, help = "Export requests and responses to files")]
    export: bool,

    #[arg(long = "export-json", action = clap::ArgAction::SetTrue, help = "Export execution results as a JSON file")]
    export_json: bool,

    #[arg(long = "include-secrets", action = clap::ArgAction::SetTrue, help = "Include sensitive headers and secret-like values in logs, reports, and exports")]
    include_secrets: bool,

    #[arg(long = "no-telemetry", action = clap::ArgAction::SetTrue, help = "Disable anonymous telemetry data collection")]
    no_telemetry: bool,

    #[arg(
        long = "delay",
        value_name = "MILLISECONDS",
        default_value_t = 0,
        help = "Delay between requests in milliseconds (default: 0)"
    )]
    delay: u64,

    #[arg(long = "fail-fast", action = clap::ArgAction::SetTrue, help = "Stop immediately on the first failed request and show its full details")]
    fail_fast: bool,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum OutputTypeArg {
    #[default]
    #[value(name = "OneRequestPerFile")]
    OneRequestPerFile,
    #[value(name = "OneFile")]
    OneFile,
    #[value(name = "OneFilePerTag")]
    OneFilePerTag,
}

impl std::fmt::Display for OutputTypeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneRequestPerFile => f.write_str("OneRequestPerFile"),
            Self::OneFile => f.write_str("OneFile"),
            Self::OneFilePerTag => f.write_str("OneFilePerTag"),
        }
    }
}

impl From<OutputTypeArg> for OutputType {
    fn from(value: OutputTypeArg) -> Self {
        match value {
            OutputTypeArg::OneRequestPerFile => OutputType::OneRequestPerFile,
            OutputTypeArg::OneFile => OutputType::OneFile,
            OutputTypeArg::OneFilePerTag => OutputType::OneFilePerTag,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum ReportFormatArg {
    #[default]
    Markdown,
    Html,
}

impl From<ReportFormatArg> for ReportFormat {
    fn from(value: ReportFormatArg) -> Self {
        match value {
            ReportFormatArg::Markdown => ReportFormat::Markdown,
            ReportFormatArg::Html => ReportFormat::Html,
        }
    }
}

fn main() -> ExitCode {
    match run_cli() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

fn run_cli() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate(args)) => run_generate(args),
        Some(Commands::Run(args)) => run_run(args),
        None => {
            Cli::command().print_help()?;
            println!();
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn run_generate(args: GenerateArgs) -> Result<ExitCode, Box<dyn std::error::Error>> {
    if args.version {
        println!("httpfiletools-generate {}", env!("CARGO_PKG_VERSION"));
        return Ok(ExitCode::SUCCESS);
    }

    let Some(openapi) = args.openapi else {
        let mut command = Cli::command();
        command
            .find_subcommand_mut("generate")
            .expect("generate command exists")
            .print_help()?;
        println!();
        return Ok(ExitCode::SUCCESS);
    };

    if args.azure_tenant_id.is_some() && args.azure_scope.is_none() {
        eprintln!(
            "Azure tenant ID was provided without an Azure scope; continuing without Azure token acquisition"
        );
    }

    if !args.no_logging {
        println!("HTTP File Generator v{}", env!("CARGO_PKG_VERSION"));
    }

    let request = GenerateRequest::with_options(
        openapi,
        GenerateOptions {
            authorization_header: args.authorization_header,
            authorization_header_from_environment_variable: args
                .load_authorization_header_from_environment,
            authorization_header_variable_name: Some(args.authorization_header_variable_name),
            content_type: Some(args.content_type),
            base_url: args.base_url,
            output_type: args.output_type.into(),
            timeout: Some(args.timeout),
            generate_intellij_tests: args.generate_intellij_tests,
            custom_headers: args.custom_headers,
            skip_headers: args.skip_headers,
            tolerate_invalid_openapi31: args.skip_validation,
        },
    );

    let result = generate_http_files(&request)?;
    let written = write_generated_files(&args.output, result.files)?;

    for path in written {
        println!("Written {}", path.display());
    }
    println!("Generation completed");

    Ok(ExitCode::SUCCESS)
}

fn write_generated_files(
    output: &Path,
    files: Vec<httpfiletools_core::HttpFile>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    fs::create_dir_all(output)?;
    let mut written = Vec::with_capacity(files.len());

    for file in files {
        let path = output.join(file.filename);
        fs::write(&path, file.content)?;
        written.push(path);
    }

    Ok(written)
}

fn run_run(args: RunArgs) -> Result<ExitCode, Box<dyn std::error::Error>> {
    if args.version {
        println!("httpfiletools-run {}", env!("CARGO_PKG_VERSION"));
        return Ok(ExitCode::SUCCESS);
    }

    if args.files.is_empty() && !args.discover {
        let mut command = Cli::command();
        command
            .find_subcommand_mut("run")
            .expect("run command exists")
            .print_help()?;
        println!();
        return Ok(ExitCode::SUCCESS);
    }

    let files = if args.discover {
        discover_http_files()?
    } else {
        args.files
    };

    let request = RunRequest::with_options(
        files,
        RunOptions {
            environment_name: args.environment,
            verbose: args.verbose,
            log_filename: args.log,
            insecure: args.insecure,
            pretty_json: args.pretty_json,
            silent: args.no_banner,
            delay_ms: args.delay,
            fail_fast: args.fail_fast,
        },
    );

    let results = run_http_files(&request)?;

    if let Some(format) = args.report {
        let report = generate_report(&results, format.into())?;
        println!("Report written to {report}");
    }

    if args.export {
        let files = export_results(&results, args.pretty_json)?;
        for file in files {
            println!("Exported {file}");
        }
    }

    if args.export_json {
        let file = export_json(&results)?;
        println!("JSON results written to {file}");
    }

    let _ = (args.include_secrets, args.no_telemetry);

    if results.success {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::from(1))
    }
}
