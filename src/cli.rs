use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dsp_meta::domain::model::error::ValidationError;
use dsp_meta::domain::model::json_schema_validator::{validate_file, SchemaVersion};
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
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum Schema {
    #[default]
    Draft,
    Final,
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

        _ => {
            println!("No subcommand found");
            Ok(())
        }
    }
}
