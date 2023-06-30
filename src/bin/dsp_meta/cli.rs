use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cmd::validate::validate;

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

    Transform {
        /// The required path to the project metadata file to operate on
        #[arg(short, long, value_name = "FILE")]
        project: PathBuf,

        /// Json output format
        #[arg(short, long)]
        json: bool,

        /// Turtle output format
        #[arg(short, long)]
        ttl: bool,
    },
}

pub fn parse() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode if off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode in on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Validate { project }) => validate(project),
        None => {}
        _ => {}
    }
}
