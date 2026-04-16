use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "helo", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register a blueprint: name + runtime
    Add {
        /// Blueprint name
        name: String,
        /// Runtime: claude, pi, or opencode
        #[arg(long)]
        runtime: String,
    },
    /// Launch a blueprint in the current directory
    Run {
        /// Blueprint name
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Blueprint {
    name: String,
    runtime: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    blueprints: Vec<Blueprint>,
}

fn config_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", "helo")
        .context("could not determine config directory")?;
    Ok(dirs.config_dir().join("config.toml"))
}

fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read {}", path.display()))?;
    toml::from_str(&text).context("could not parse config.toml")
}

fn save(cfg: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    std::fs::write(&path, toml::to_string_pretty(cfg)?)
        .with_context(|| format!("could not write {}", path.display()))
}

fn env_dir(runtime: &str, name: &str) -> PathBuf {
    let prefix = match runtime {
        "claude"   => ".claude-env",
        "pi"       => ".pi-env",
        "opencode" => ".opencode-env",
        other      => return PathBuf::from(format!(".{other}-env-{name}")),
    };
    PathBuf::from(format!("{prefix}-{name}"))
}

fn isolation_var(runtime: &str) -> &'static str {
    match runtime {
        "claude"   => "CLAUDE_CONFIG_DIR",
        "pi"       => "PI_CODING_AGENT_DIR",
        "opencode" => "OPENCODE_CONFIG",
        _          => "AGENT_CONFIG_DIR",
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { name, runtime } => {
            let mut cfg = load()?;
            if cfg.blueprints.iter().any(|b| b.name == name) {
                bail!("blueprint '{name}' already exists");
            }
            cfg.blueprints.push(Blueprint { name: name.clone(), runtime });
            save(&cfg)?;
            println!("Added '{name}'.");
        }

        Commands::Run { name } => {
            let cfg = load()?;
            let bp = cfg.blueprints.iter()
                .find(|b| b.name == name)
                .with_context(|| format!("no blueprint named '{name}' — run: helo add {name} --runtime <runtime>"))?;

            let cwd = std::env::current_dir()?;
            let dir = cwd.join(env_dir(&bp.runtime, &name));
            std::fs::create_dir_all(&dir)
                .with_context(|| format!("could not create {}", dir.display()))?;

            let status = std::process::Command::new(&bp.runtime)
                .env(isolation_var(&bp.runtime), &dir)
                .status()
                .with_context(|| format!("could not launch '{}' — is it installed and on PATH?", bp.runtime))?;

            std::process::exit(status.code().unwrap_or(1));
        }
    }
    Ok(())
}
