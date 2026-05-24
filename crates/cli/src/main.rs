use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand, ValueEnum};
use httpfiletools_generator::{GenerateError, GenerateOptions, GenerateOutputType};
use httpfiletools_runner::{ReportFormat, RunError, RunOptions, RunStatus};
use httprunner_core::logging::get_support_key;

#[derive(Debug, Parser)]
#[command(name = "httpfiletools", version, about = "Generate and run .http files")]
struct Cli {
    #[arg(global = true, long, help = "Disable the default support-key banner")]
    no_logging: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Generate(GenerateCommand),
    Run(RunCommand),
    Version,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputTypeArg {
    OneRequestPerFile,
    OneFile,
    OneFilePerTag,
}

impl From<OutputTypeArg> for GenerateOutputType {
    fn from(value: OutputTypeArg) -> Self {
        match value {
            OutputTypeArg::OneRequestPerFile => GenerateOutputType::OneRequestPerFile,
            OutputTypeArg::OneFile => GenerateOutputType::OneFile,
            OutputTypeArg::OneFilePerTag => GenerateOutputType::OneFilePerTag,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ReportFormatArg {
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

#[derive(Debug, Parser)]
struct GenerateCommand {
    spec: String,
    #[arg(long, default_value = ".")]
    output: PathBuf,
    #[arg(long, value_enum, default_value_t = OutputTypeArg::OneRequestPerFile)]
    output_type: OutputTypeArg,
    #[arg(long)]
    base_url: Option<String>,
    #[arg(long, default_value = "application/json")]
    content_type: String,
    #[arg(long)]
    authorization_header: Option<String>,
    #[arg(long)]
    load_authorization_header_from_environment: bool,
    #[arg(long, default_value = "authorization")]
    authorization_header_variable_name: String,
    #[arg(long)]
    azure_scope: Option<String>,
    #[arg(long)]
    azure_tenant_id: Option<String>,
    #[arg(long = "custom-header")]
    custom_headers: Vec<String>,
    #[arg(long)]
    skip_headers: bool,
    #[arg(long)]
    skip_validation: bool,
    #[arg(long)]
    generate_intellij_tests: bool,
    #[arg(long, default_value_t = 120)]
    timeout: u64,
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Parser)]
struct RunCommand {
    paths: Vec<PathBuf>,
    #[arg(long)]
    discover: bool,
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    pretty_json: bool,
    #[arg(long, default_value_t = 0)]
    delay: u64,
    #[arg(long = "env")]
    environment: Option<String>,
    #[arg(long)]
    insecure: bool,
    #[arg(long, num_args = 0..=1, default_missing_value = "httpfiletools.log")]
    log: Option<PathBuf>,
    #[arg(long, num_args = 0..=1, value_enum, default_missing_value = "markdown")]
    report: Option<ReportFormatArg>,
    #[arg(long, num_args = 0..=1, default_missing_value = "exports")]
    export: Option<PathBuf>,
    #[arg(long)]
    fail_fast: bool,
    #[arg(long)]
    no_banner: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if !cli.no_logging {
        print_support_key();
    }

    match cli.command {
        Commands::Generate(command) => run_generate(command),
        Commands::Run(command) => run_run(command),
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
    }
}

fn run_generate(command: GenerateCommand) -> ExitCode {
    let options = GenerateOptions {
        spec: command.spec,
        output: command.output,
        output_type: command.output_type.into(),
        base_url: command.base_url,
        content_type: command.content_type,
        authorization_header: command.authorization_header,
        load_authorization_header_from_environment: command
            .load_authorization_header_from_environment,
        authorization_header_variable_name: command.authorization_header_variable_name,
        azure_scope: command.azure_scope,
        azure_tenant_id: command.azure_tenant_id,
        custom_headers: command.custom_headers,
        skip_headers: command.skip_headers,
        skip_validation: command.skip_validation,
        generate_intellij_tests: command.generate_intellij_tests,
        timeout_seconds: command.timeout,
        dry_run: command.dry_run,
    };

    match httpfiletools_generator::generate(&options) {
        Ok(result) => {
            if result.dry_run {
                println!("Dry run: {} file(s) would be written", result.files.len());
            } else {
                println!("Generated {} file(s)", result.files.len());
            }
            for file in result.files {
                println!("- {}", file.destination.display());
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            if error.is_usage_error() {
                ExitCode::from(2)
            } else if matches!(error, GenerateError::ParseBackValidation { .. } | GenerateError::OpenApiLoad(_)) {
                ExitCode::from(3)
            } else {
                ExitCode::from(4)
            }
        }
    }
}

fn run_run(command: RunCommand) -> ExitCode {
    let options = RunOptions {
        paths: command.paths,
        discover: command.discover,
        verbose: command.verbose,
        pretty_json: command.pretty_json,
        delay_ms: command.delay,
        environment: command.environment,
        insecure: command.insecure,
        log: command.log,
        report: command.report.map(Into::into),
        export: command.export,
        fail_fast: command.fail_fast,
        no_banner: command.no_banner,
    };

    if !options.no_banner {
        println!("httpfiletools run");
    }

    match httpfiletools_runner::run(&options) {
        Ok(result) => {
            print!(
                "{}",
                httpfiletools_runner::format_console_output(&result.results, options.verbose)
            );
            if let Some(path) = result.report_path {
                println!("Report: {}", path.display());
            }
            if let Some(path) = result.log_path {
                println!("Log: {}", path.display());
            }
            if !result.exported_files.is_empty() {
                println!("Exported {} artifact(s)", result.exported_files.len());
            }
            match result.status {
                RunStatus::Success => ExitCode::SUCCESS,
                RunStatus::AssertionFailure => ExitCode::from(1),
            }
        }
        Err(error) => {
            eprintln!("{error}");
            match error {
                RunError::NoFiles => ExitCode::from(2),
                RunError::Parse { .. } => ExitCode::from(3),
                RunError::Runtime(_) | RunError::Io(_) => ExitCode::from(4),
            }
        }
    }
}

fn print_support_key() {
    if let Ok(support_key) = get_support_key() {
        println!("Support key: {}", support_key.short_key);
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn help_lists_subcommands() {
        Command::cargo_bin("httpfiletools")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("generate"))
            .stdout(predicate::str::contains("run"));
    }
}
