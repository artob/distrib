// This is free and unencumbered software released into the public domain.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::Utf8PathBuf,
    crates::clap::{Parser, Subcommand},
};
use distrib::{LoadError, Package, PackageKind, PackageManager, PackageRegistry, Tool};
use glob::glob;
use std::env::set_current_dir;
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Distrib helps you distribute your software.
#[derive(Debug, Parser)]
#[command(name = "Distrib", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The working directory to use.
    #[clap(short = 'C', long, default_value = ".", global = true)]
    cwd: Option<Utf8PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inspect the current package's metadata.
    #[clap(aliases = ["describe"])]
    Inspect {
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
            output: "json".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("unknown --output format: {0}")]
    UnknownOutputFormat(String),

    #[error("missing --with tool (auto-detection failed)")]
    MissingWithTool,

    #[error("unknown --with tool: {0}")]
    UnknownWithTool(PackageManager),

    #[error("missing --to registry (auto-detection failed)")]
    MissingToRegistry,

    #[error(transparent)]
    LoadPackage(#[from] LoadError),

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

    if let Some(cwd) = options.cwd {
        debug!("Changing the current directory to `{}`...", cwd);
        set_current_dir(&cwd)?;
        info!("Changed the current directory to `{}`.", cwd);
    };

    let mut packages: Vec<Package> = vec![];
    for package_kind in PackageKind::ALL {
        let manifest_name = package_kind.manifest_name();
        for manifest_path in glob(manifest_name).unwrap().filter_map(Result::ok) {
            let manifest_path = Utf8PathBuf::try_from(manifest_path).unwrap();
            debug!(
                "Loading the {} manifest `{}`...",
                package_kind, manifest_path
            );
            match Package::load(&manifest_path, Some(package_kind.clone())) {
                Ok(package) => {
                    packages.push(package);
                    info!("Loaded the {} manifest `{}`.", package_kind, manifest_path);
                },
                Err(err) => {
                    warn!(
                        "Failed to load the {} manifest `{}`: {}",
                        package_kind, manifest_path, err
                    );
                },
            }
        }
    }

    if packages.is_empty() {
        return Err(Other("no packages found".into()));
    }

    let command = options.command.unwrap_or_default();
    for package in packages {
        match command {
            Command::Inspect { ref output } => match output.as_str() {
                "json" => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&package).map_err(|e| Other(e.into()))?
                    );
                },
                _ => {
                    return Err(UnknownOutputFormat(output.clone()));
                },
            },

            Command::Clean { ref with } => {
                let with = with
                    .clone()
                    .or_else(|| package.tool())
                    .ok_or(MissingWithTool)?;
                let program = tool_for(with)?;
                let _ = program.clean()?;
            },

            Command::Build { ref with } => {
                let with = with
                    .clone()
                    .or_else(|| package.tool())
                    .ok_or(MissingWithTool)?;
                let program = tool_for(with)?;
                let _ = program.build()?;
            },

            Command::Publish { ref with, ref to } => {
                let with = with
                    .clone()
                    .or_else(|| package.tool())
                    .ok_or(MissingWithTool)?;
                let to = to
                    .clone()
                    .or_else(|| package.registry())
                    .ok_or(MissingToRegistry)?;
                let program = tool_for(with)?;
                let _ = program.publish(Some(to))?;
            },
        }
    }

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
