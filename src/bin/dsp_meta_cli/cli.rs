use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dsp_meta::error::DspMetaError;
use dsp_meta::operation::convert::convert;
use dsp_meta::operation::validate::validate;
use log::info;

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
        /// The required path to the project metadata file to operate on
        #[arg(short, long, value_name = "FILE")]
        project: PathBuf,
    },

    Convert {
        /// The required path to the source file
        source: PathBuf,

        /// The required path to the target file
        target: PathBuf,
    },
}

pub fn parse() -> Result<(), DspMetaError> {
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
        Some(Commands::Validate { project }) => validate(project),
        Some(Commands::Convert { source, target }) => convert(source, target),
        None => Ok(()),
    }
}
