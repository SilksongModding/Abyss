// main does not return a Result; detection logic is in detector
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{Level, error, info};
mod detector;
mod provider;
use detector::Detector;
use provider::RealSteamProvider;

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
    }
}

// Detection logic refactored into `detector` module
