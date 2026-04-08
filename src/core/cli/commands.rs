// src/core/cli/commands.rs
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(name = "subhuti")]
#[command(about = "Subhuti 个人自动化流水线引擎 (Rust Core)")]
pub struct SubhutiCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "pipeline", alias = "p")]
    Pipeline {
        #[arg(short, long, value_name = "FILE")]
        template: PathBuf,
        #[arg(short, long, value_name = "KEY=VALUE", number_of_values = 1)]
        var: Vec<String>,
    },

    #[command(name = "task", alias = "t")]
    Task {
        #[arg(value_name = "DESCRIPTION")]
        description: String,
        #[arg(short, long)]
        interactive: bool,
    },

    #[command(name = "plugin", alias = "pl")]
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },

    #[command(name = "list-skills", alias = "ls")]
    ListSkills,

    #[command(name = "history", alias = "h")]
    History {
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum PluginCommands {
    List,
    Load { path: PathBuf },
    Unload { name: String },
}