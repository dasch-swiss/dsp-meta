use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dsp_meta::domain::model::error::ValidationError;
use dsp_meta::domain::model::json_schema_validator::{validate_file, SchemaVersion};
use dsp_meta::domain::url_checker::UrlChecker;
use log::info;
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing thing
    Validate {
        #[arg(short, long, default_value = "draft")]
        schema: Schema,
        /// The required path to the project metadata file to operate on
        #[arg(short, long, value_name = "FILE")]
        project: PathBuf,
    },
    /// Check all URLs in JSON metadata files
    CheckUrls {
        /// The directory containing JSON metadata files
        #[arg(short, long, value_name = "DIR")]
        data_dir: PathBuf,
        /// Output format
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
        /// Request timeout in seconds
        #[arg(short, long, default_value = "10")]
        timeout: u64,
    },
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum Schema {
    #[default]
    Draft,
    Final,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum OutputFormat {
    #[default]
    Text,
    Json,
    Markdown,
}
impl Schema {
    fn as_schema_version(&self) -> SchemaVersion {
        match self {
            Schema::Draft => SchemaVersion::Draft,
            Schema::Final => SchemaVersion::Final,
        }
    }
}
impl TryFrom<String> for Schema {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "draft" => Ok(Schema::Draft),
            "final" => Ok(Schema::Final),
            _ => Err("Invalid schema version".to_string()),
        }
    }
}

pub fn parse() -> Result<(), String> {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => info!("Debug mode if off"),
        1 => info!("Debug mode is kind of on"),
        2 => info!("Debug mode in on"),
        _ => info!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Validate { project, schema }) => {
            let schema = schema.as_schema_version();
            println!(
                "Validating file {:?} against {:?} schema.",
                project, &schema
            );
            println!();
            match validate_file(project, schema) {
                Ok(state) => {
                    if state.is_valid() {
                        println!("Validation passed.");
                        Ok(())
                    } else {
                        println!("ERRORS:");
                        for error in state.errors {
                            println!(" * {error:?}")
                        }
                        println!();
                        Err("Validation failed".to_string())
                    }
                }
                Err(e) => match e {
                    ValidationError::FileNotLoaded(_) => Err("File not found.".to_string()),
                    ValidationError::SchemaError(_) => Err("Schema error.".to_string()),
                    ValidationError::NotAJsonFile(_) => Err("Not a JSON file.".to_string()),
                },
            }
        }

        Some(Commands::CheckUrls {
            data_dir,
            format,
            timeout,
        }) => {
            println!("Checking URLs in directory: {}", data_dir.display());
            println!("Timeout: {}s", timeout);
            println!();

            let checker = UrlChecker::new(Some(*timeout))
                .map_err(|e| format!("Failed to create URL checker: {}", e))?;

            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

            let report = runtime
                .block_on(checker.check_directory(data_dir))
                .map_err(|e| format!("Failed to check URLs: {}", e))?;

            match format {
                OutputFormat::Text => report.print_text(),
                OutputFormat::Json => match report.print_json() {
                    Ok(json) => println!("{}", json),
                    Err(e) => return Err(format!("Failed to serialize JSON: {}", e)),
                },
                OutputFormat::Markdown => report.print_markdown(),
            }

            // Always return Ok() - report is generated regardless of broken links
            // This allows the command to succeed while still showing issues
            Ok(())
        }

        _ => {
            println!("No subcommand found");
            Ok(())
        }
    }
}
