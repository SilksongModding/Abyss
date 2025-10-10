// main does not return a Result; detection logic is in detector
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{Level, error, info};
mod bepinex_handler;
mod bepinex_provider;
mod detector;
mod provider;
use detector::Detector;
use provider::RealSteamProvider;

use crate::bepinex_handler::BepinexHandler;

// SILKSONG_APP_ID is available from the library crate root

// NOTE: name hints and detection logic live in `detector` module

#[derive(Parser, Debug)]
#[command(
    name = "abyss",
    version,
    about = "Wrapper around BepInEx, powering Silksong mods"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Detect the Silksong installation directory
    Detect {
        /// Manual override of the game directory (highest precedence)
        #[arg(long, value_name = "PATH")]
        game_dir: Option<PathBuf>,

        /// Optional Steam AppID (for debugging)
        #[arg(long, value_name = "APPID")]
        app_id: Option<u32>,

        /// Extra folder name hints to search under steamapps/common
        #[arg(long = "name-hint", value_name = "NAME", num_args = 0.., action = clap::ArgAction::Append)]
        name_hints: Vec<String>,

        /// Only exit code indicates success/failure (no stdout on success)
        #[arg(long)]
        quiet: bool,
    },

    /// Checks if BepInEx is installed
    Check {
        /// Manual override of the game directory (highest precedence)
        #[arg(long, value_name = "PATH")]
        game_dir: Option<PathBuf>,

        /// Optional Steam AppID (for debugging)
        #[arg(long, value_name = "APPID")]
        app_id: Option<u32>,

        /// Extra folder name hints to search under steamapps/common
        #[arg(long = "name-hint", value_name = "NAME", num_args = 0.., action = clap::ArgAction::Append)]
        name_hints: Vec<String>,
    },

    Install {
        /// Manual override of the game directory (highest precedence)
        #[arg(long, value_name = "PATH")]
        game_dir: Option<PathBuf>,

        /// Optional Steam AppID (for debugging)
        #[arg(long, value_name = "APPID")]
        app_id: Option<u32>,

        /// Extra folder name hints to search under steamapps/common
        #[arg(long = "name-hint", value_name = "NAME", num_args = 0.., action = clap::ArgAction::Append)]
        name_hints: Vec<String>,
    },
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter("info")
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Detect {
            game_dir,
            app_id,
            name_hints,
            quiet,
        } => {
            let detector = Detector::new(RealSteamProvider);
            match detector.detect_game_dir(game_dir.as_deref(), app_id, &name_hints) {
                Ok(path) => {
                    if !quiet {
                        println!("{}", path.display());
                    }
                    info!("Game directory detected: {}", path.display());
                }
                Err(err) => {
                    error!("{err:#}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Check {
            game_dir,
            app_id,
            name_hints,
        } => {
            println!("TODO: check command");
            let detector = Detector::new(RealSteamProvider);

            let game_path = match detector.detect_game_dir(game_dir.as_deref(), app_id, &name_hints)
            {
                Ok(value) => value,
                Err(err) => {
                    error!("{err:#}");
                    std::process::exit(1);
                }
            };

            let handler = BepinexHandler::new(game_path);
            match handler.check_installation() {
                Ok(true) => println!("BepInEx installation found"),
                Ok(false) => println!(
                    "BepInEx installation not found, you can install it with 'abyss install'"
                ),
                Err(err) => {
                    error!("{err:#}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Install {
            game_dir,
            app_id,
            name_hints,
        } => {
            let detector = Detector::new(RealSteamProvider);
            let game_path = match detector.detect_game_dir(game_dir.as_deref(), app_id, &name_hints)
            {
                Ok(value) => value,
                Err(err) => {
                    error!("{err:#}");
                    std::process::exit(1);
                }
            };

            let handler = BepinexHandler::new(game_path);
            match handler.install() {
                Ok(_) => println!("BepInEx successfully installed"),
                Err(err) => {
                    error!("{err:#}");
                    std::process::exit(1);
                }
            }
        }
    }
}

// Detection logic refactored into `detector` module
