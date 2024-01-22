use std::path::PathBuf;

use clap::{ValueEnum, Parser, CommandFactory, error::ErrorKind};
use forge_lib::get_file_contents;
use tracing::{metadata::LevelFilter, info, debug, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod linker;

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
    about = "Links 6502 object files into a final binary"
)]
struct Cli {
    /// The level of verbosity to use
    #[arg(short, long)]
    verbose: Option<VerboseLevels>,

    /// The linker script to use
    #[arg(short, long)]
    linker: PathBuf,

    /// The list of out files generated by the assembler
    input: Vec<PathBuf>,
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

    if cli.input.len() == 0 {
        let mut cmd = Cli::command();
        cmd.error(ErrorKind::MissingRequiredArgument, "Missing input files. Please provide at least 1").exit();
    }

    info!("Starting linker");

    for file in cli.input {
        let file_contents = match get_file_contents(&file) {
            Ok(file) => file,
            Err(e) => {
                error!("{}", e);
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };
        debug!("{:?}", file_contents);
    }
}