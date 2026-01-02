#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mhf_iel::MhfConfig;
use std::{fs::File, path::PathBuf, process::exit};
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(about = Some("Runs MHF. Config data can be specified through arguments, and defaults to a 'config.json' file in the current folder."))]
pub struct CliConfig {
    #[arg(long, help = "JSON config file")]
    pub config_file: Option<PathBuf>,

    #[arg(long, help = "JSON config data")]
    pub config_data: Option<String>,
}

fn main() {
    // Parse command-line arguments
    let cli_config = CliConfig::try_parse().unwrap_or_else(|e| {
        eprintln!("❌ [CLI] Argument parsing error: {e}");
        exit(1);
    });

    // Load config data from file or CLI argument
    let config_data = cli_config
        .config_data
        .or_else(|| {
            cli_config
                .config_file
                .or_else(|| std::env::current_dir().map(|d| d.join("config.json")).ok())
                .and_then(|v| {
                    eprintln!("📄 [Config] Loading from: {}", v.display());
                    File::open(v).ok()
                })
                .and_then(|v| std::io::read_to_string(v).ok())
        })
        .unwrap_or_else(|| {
            eprintln!("❌ [Config] Unable to locate 'config.json' file");
            exit(2);
        });

    // Parse JSON config
    let mhf_config: MhfConfig = serde_json::from_str(&config_data).unwrap_or_else(|e| {
        eprintln!("❌ [Config] JSON parsing error: {}", e);
        exit(3);
    });

    // Log font configuration (if present)
    if let Some(ref font_name) = mhf_config.font_name {
        eprintln!("🔤 [Config] Custom font specified: {}", font_name);
    }

    // Log friends count
    eprintln!("👥 [Config] Friends in config: {}", mhf_config.friends.len());

    // Run the game
    eprintln!("🚀 [Main] Starting MHF...\n");
    let result = mhf_iel::run(mhf_config);

    // Handle result
    if let Err(e) = result {
        eprintln!("\n❌ [Main] Error running MHF: {}", e);
        exit(4);
    }

    eprintln!("\n✅ [Main] MHF terminated successfully");
    exit(0);
}
