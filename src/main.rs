use clap::Parser;
use tracing::{Level, info};
use tracing_subscriber;

/// Abyss: Mod loader for Hollow Knight Silksong
#[derive(Parser, Debug)]
#[command(name = "Abyss")]
#[command(about = "Wrapper around BepInEx, powering Silksong mods", long_about = None)]
struct Cli {
    /// Print version
    #[arg(short, long)]
    version: bool,
}

fn main() {
    // Setup logger
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cli = Cli::parse();

    if cli.version {
        println!("Abyss v0.1.0");
        return;
    }

    info!("🚀 Abyss mod loader ready!");
}
