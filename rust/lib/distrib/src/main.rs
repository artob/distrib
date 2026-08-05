// This is free and unencumbered software released into the public domain.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::Utf8PathBuf,
    crates::clap::{Parser, Subcommand},
};
use distrib::{Package, PackageManager, PackageRegistry, Tool};
use thiserror::Error;
use tracing::error;

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
        /// The working directory to use.
        #[clap(short = 'C', long, default_value = ".")]
        cwd: Option<Utf8PathBuf>,

        /// The output format to use.
        #[clap(short, long, default_value = "json")]
        output: String,
    },

    /// Remove the current package's build artifacts.
    Clean {
        /// The package manager to clean with [default: auto].
        #[clap(short, long)]
        with: Option<PackageManager>,
    },

    /// Build the current package.
    Build {
        // TODO: --for PackageEcosystem
        /// The package manager to build with [default: auto].
        #[clap(short, long)]
        with: Option<PackageManager>,
    },

    /// Publish the current package to a package registry.
    Publish {
        /// The package manager to build with [default: auto].
        #[clap(short, long)]
        with: Option<PackageManager>,

        /// The package registry to publish to [default: auto].
        #[clap(short, long)]
        to: Option<PackageRegistry>,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Inspect {
            cwd: None,
            output: "json".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("unknown --output format: {0}")]
    UnknownOutputFormat(String),

    #[error("unknown --with tool: {0}")]
    UnknownWithTool(PackageManager),

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

    let result = Ok(());

    match options.command.unwrap_or_default() {
        Command::Inspect { cwd, output } => {
            let cwd = cwd.unwrap_or(".".into());
            let package = Package::locate(&cwd).unwrap();
            match output.as_str() {
                "json" => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&package).map_err(|e| Other(e.into()))?
                    );
                },
                _ => {
                    return Err(UnknownOutputFormat(output));
                },
            }
        },

        Command::Clean { with } => {
            let with = with.unwrap_or(PackageManager::Cargo); // TODO: auto
            let program = tool_for(with)?;
            let _ = program.clean()?;
        },

        Command::Build { with } => {
            let with = with.unwrap_or(PackageManager::Cargo); // TODO: auto
            let program = tool_for(with)?;
            let _ = program.build()?;
        },

        Command::Publish { with, to } => {
            let with = with.unwrap_or(PackageManager::Cargo); // TODO: auto
            let to = to.unwrap_or(PackageRegistry::Crates); // TODO: auto
            let program = tool_for(with)?;
            let _ = program.publish(Some(to))?;
        },
    };

    result
}

fn tool_for(input: PackageManager) -> Result<Box<dyn Tool>, ProgramError> {
    use distrib::PackageManager::*;
    Ok(match input {
        #[cfg(feature = "rust")]
        Cargo => Box::new(distrib_rust::CargoProgram::default()),
        #[cfg(feature = "jsr")]
        Jsr => Box::new(distrib_jsr::JsrProgram::default()),
        //#[cfg(feature = "mix")]
        //Mix => Some(Box::new(distrib_rust::MixProgram::default())),
        #[cfg(feature = "npm")]
        Npm => Box::new(distrib_npm::NpmProgram::default()),
        #[cfg(feature = "dart")]
        Pub => Box::new(distrib_dart::DartProgram::default()),
        #[cfg(feature = "python")]
        PyPi => Box::new(distrib_python::PipProgram::default()),
        #[cfg(feature = "ruby")]
        RubyGems => Box::new(distrib_ruby::GemProgram::default()),
        _ => return Err(ProgramError::UnknownWithTool(input)),
    })
}
