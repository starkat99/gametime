use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Command line args
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// Cli commands
#[derive(Debug, Subcommand)]
enum Command {
    /// Package a gametime ROM
    #[command(
        short_flag = 'p',
        long_flag = "package",
        visible_alias = "build",
        short_flag_alias = 'b',
        long_flag_alias = "build"
    )]
    Package {
        /// Package manifest file override
        #[arg(value_name = "FILE")]
        manifest: Option<PathBuf>,

        /// Package output file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Package {
            manifest: _,
            output: _,
        } => {}
    }
}
