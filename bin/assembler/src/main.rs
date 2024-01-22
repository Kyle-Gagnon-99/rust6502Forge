use std::{path::PathBuf, fs};

use clap::{ValueEnum, Parser, command, Subcommand};
use forge_lib::line::Line;
use scanner::Scanner;
use tracing::{metadata::LevelFilter, info, debug};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::process::{process_file, process_lines};

mod scanner;
mod error;
mod process;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum VerboseLevels {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Parser)]
#[command(
    author = "Kyle Gagnon",
    version = "0.1.0",
    about = "Parses a 6502 assembly file into an object file to be linked"
)]
struct Cli {
    /// The input assembly file; Required
    input: PathBuf,

    /// The output file (with extension)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// The level of verbosity to use
    #[arg(short, long)]
    verbose: Option<VerboseLevels>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Assembles a file into a final executable without linking
    Exe
}

fn main() {
    let cli = Cli::parse();

    let level = match cli.verbose {
        Some(VerboseLevels::Trace) => LevelFilter::TRACE,
        Some(VerboseLevels::Debug) => LevelFilter::DEBUG,
        Some(VerboseLevels::Info) => LevelFilter::INFO,
        Some(VerboseLevels::Warn) => LevelFilter::WARN,
        Some(VerboseLevels::Error) => LevelFilter::ERROR,
        None => LevelFilter::OFF,
    };

    let filter = EnvFilter::from_default_env().add_directive(level.into());

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("{:?}", cli.input);

    let file_contents = convert_file_to_string(&cli.input);
    let mut parsed_file = parse_file(file_contents);

    // If there is something for the out_file then use that, otherwise just generate the same file but replace the file extension
    let output_file = match cli.output {
        Some(output) => output,
        None => {
            // Get the name of the file without the extension
            let mut output = cli.input.clone();
            output.set_extension("out");
            output
        }
    };

    if cli.command.is_some() {
        match cli.command.unwrap() {
            Commands::Exe => {
                let bytes = process_lines(&mut parsed_file);
            }
        }
    }

    //let _ = process_file(&mut parsed_file, &cli.input, &output_file);
}

fn convert_file_to_string(file_path: &PathBuf) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn parse_file(file_contents: String) -> Vec<Line> {
    let mut scanner = Scanner::new(&file_contents);
    let mut line_list = Vec::new();
    while !scanner.is_done() {
        let line = match scanner.line() {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{} at line {}", e, scanner.lines + 1);
                std::process::exit(1);
            }
        };
        line_list.push(line);
    }

    line_list
}


