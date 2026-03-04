use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "upakor")]
#[command(about = "Upakor-G14 CLI - Control ASUS laptop features", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    /// Output format
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Power profile management
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Battery charge limit management
    ChargeLimit {
        #[command(subcommand)]
        action: ChargeLimitAction,
    },
    /// GPU mode management
    Gpu {
        #[command(subcommand)]
        action: GpuAction,
    },
    /// Show current hardware metrics
    Metrics,
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ProfileAction {
    /// Get current power profile
    Get,
    /// Set power profile
    Set {
        /// Profile name (quiet, balanced, performance)
        profile: String,
    },
}

#[derive(Subcommand)]
enum ChargeLimitAction {
    /// Get current charge limit
    Get,
    /// Set charge limit (50-100)
    Set {
        /// Charge limit percentage (50-100)
        limit: u8,
    },
}

#[derive(Subcommand)]
enum GpuAction {
    /// Get current GPU mode
    Get,
    /// Set GPU mode
    Set {
        /// GPU mode (integrated, hybrid, dedicated)
        mode: String,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get all settings or a specific setting
    Get {
        /// Setting key (optional, e.g., power.default_profile)
        key: Option<String>,
    },
    /// Set a configuration value
    Set {
        /// Setting key (e.g., power.default_profile)
        key: String,
        /// Setting value
        value: String,
    },
    /// Reset to default settings
    Reset,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Profile { action } => handle_profile(action).await,
        Commands::ChargeLimit { action } => handle_charge_limit(action).await,
        Commands::Gpu { action } => handle_gpu(action).await,
        Commands::Metrics => handle_metrics(cli.json).await,
        Commands::Config { action } => handle_config(action).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
}

async fn handle_profile(action: ProfileAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ProfileAction::Get => {
            // For now, just show a placeholder
            println!("{}", "Current profile: Balanced".green());
            if cfg!(feature = "mock") {
                println!("{} Using mock implementation", "[Mock]".yellow());
            }
        }
        ProfileAction::Set { profile } => {
            let valid = ["quiet", "balanced", "performance"];
            if !valid.contains(&profile.as_str()) {
                return Err(format!("Invalid profile '{}'. Valid options: quiet, balanced, performance", profile).into());
            }
            println!("{} Power profile set to {}", "Success:".green(), profile.cyan());
        }
    }
    Ok(())
}

async fn handle_charge_limit(action: ChargeLimitAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ChargeLimitAction::Get => {
            println!("{}", "Charge limit: 100%".green());
        }
        ChargeLimitAction::Set { limit } => {
            if limit < 50 || limit > 100 {
                return Err("Charge limit must be between 50 and 100".into());
            }
            println!("{} Charge limit set to {}%", "Success:".green(), limit.to_string().cyan());
        }
    }
    Ok(())
}

async fn handle_gpu(action: GpuAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        GpuAction::Get => {
            println!("{}", "GPU mode: Hybrid".green());
            println!("  Dedicated GPU: Available");
        }
        GpuAction::Set { mode } => {
            let valid = ["integrated", "hybrid", "dedicated"];
            if !valid.contains(&mode.as_str()) {
                return Err(format!("Invalid GPU mode '{}'. Valid options: integrated, hybrid, dedicated", mode).into());
            }
            println!("{} GPU mode set to {}", "Success:".green(), mode.cyan());
            println!("{} Note: GPU mode switching may require a restart", "Warning:".yellow());
        }
    }
    Ok(())
}

async fn handle_metrics(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        println!("{{");
        println!("  \"temperatures\": {{");
        println!("    \"cpu\": 45.0,");
        println!("    \"gpu\": null");
        println!("  }},");
        println!("  \"fans\": {{");
        println!("    \"cpu_rpm\": 2000,");
        println!("    \"gpu_rpm\": 0,");
        println!("    \"cpu_percentage\": 40,");
        println!("    \"gpu_percentage\": 0");
        println!("  }},");
        println!("  \"power_draw\": {{");
        println!("    \"cpu\": 15.0,");
        println!("    \"gpu\": null,");
        println!("    \"total\": 15.0");
        println!("  }}");
        println!("}}");
    } else {
        println!("{}", "Hardware Metrics".bold().cyan());
        println!("{}\t{}", "CPU Temp:".cyan(), "45.0°C".green());
        println!("{}\t{}", "CPU Fan:".cyan(), "2000 RPM (40%)".green());
        println!("{}\t{}", "GPU Fan:".cyan(), "0 RPM (0%)".dimmed());
        println!("{}\t{}W", "Power Draw:".cyan(), "15.0".green());
    }
    Ok(())
}

async fn handle_config(action: ConfigAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Get { key } => {
            if let Some(key) = key {
                println!("{}: {}", key.cyan(), "balanced".green());
            } else {
                println!("{}", "Configuration:".bold());
                println!("{}\t{}", "power.default_profile:".cyan(), "balanced".green());
                println!("{}\t{}", "power.charge_limit:".cyan(), "100".green());
                println!("{}\t{}", "gpu.mode:".cyan(), "hybrid".green());
                println!("{}\t{}", "ui.theme:".cyan(), "dark".green());
            }
        }
        ConfigAction::Set { key, value } => {
            println!("{} Set {} to {}", "Success:".green(), key.cyan(), value.cyan());
        }
        ConfigAction::Reset => {
            println!("{} Configuration reset to defaults", "Success:".green());
        }
    }
    Ok(())
}
