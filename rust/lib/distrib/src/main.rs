// This is free and unencumbered software released into the public domain.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::Utf8PathBuf,
    crates::clap::{Parser, Subcommand},
};
use thiserror::Error;
use tracing::{error, info, warn};

/// Distrib helps you distribute your software.
#[derive(Debug, Parser)]
#[command(name = "Distrib", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inspect the current package's metadata.
    #[clap(aliases = ["describe"])]
    Inspect {
        /// The project directory to use [default: $PWD].
        project: Option<Utf8PathBuf>,

        /// The output format to use.
        #[clap(short, long, default_value = "json")]
        output: String,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Inspect {
            project: None,
            output: "json".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("unknown --output format: {0}")]
    UnknownOutputFormat(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Exit(#[from] SysexitsError),

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

impl From<ProgramError> for SysexitsError {
    fn from(error: ProgramError) -> Self {
        use ProgramError::*;
        match error {
            Exit(code) => code,
            _ => EX_SOFTWARE,
        }
    }
}

pub fn main() -> SysexitsError {
    use ProgramError::*;

    match run() {
        Ok(()) => EX_OK,
        Err(Exit(exit)) => exit,
        Err(error) => {
            // TODO: color coding
            error!("{}: error: {}", env!("CARGO_PKG_NAME"), error);
            error.into()
        },
    }
}

pub fn run() -> Result<(), ProgramError> {
    use ProgramError::*;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Print the program version, if requested:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Print the program license, if requested:
    if options.flags.license {
        print!("{}", include_str!("../../../UNLICENSE"));
        return Ok(());
    }

    // Configure debug output:
    if options.flags.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .compact()
            .without_time()
            .with_target(false)
            .with_level(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // respects RUST_LOG
            .with_max_level(match options.flags.verbose {
                0 => tracing::Level::ERROR,
                1 => tracing::Level::WARN,
                2 => tracing::Level::INFO,
                3 => tracing::Level::DEBUG,
                _ => tracing::Level::TRACE,
            })
            .init();
    }

    let mut result = Ok(());

    match options.command.unwrap_or_default() {
        Command::Inspect { project: _, output } => {
            // TODO: implement `distrib inspect`

            match output.as_str() {
                "json" => {
                    // TODO: implement JSON output
                },
                _ => {
                    return Err(UnknownOutputFormat(output));
                },
            }
        },
    };

    result
}
