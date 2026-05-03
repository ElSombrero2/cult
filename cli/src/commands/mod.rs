use clap::{Parser, Subcommand};

pub mod projects;
pub mod environment;

///{n}
///{n} ██████╗██╗   ██╗██╗  ████████╗██████╗ ███████╗
///{n}██╔════╝██║   ██║██║  ╚══██╔══╝██╔══██╗██╔════╝
///{n}██║     ██║   ██║██║     ██║   ██████╔╝███████╗
///{n}██║     ██║   ██║██║     ██║   ██╔══██╗╚════██║
///{n}╚██████╗╚██████╔╝███████╗██║██╗██║  ██║███████║
///{n} ╚═════╝ ╚═════╝ ╚══════╝╚═╝╚═╝╚═╝  ╚═╝╚══════╝
///{n}
/// local environments store
///{n} https://github.com/ElSombrero2/cult.git
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CultCli {
    #[command(subcommand)]
    pub command: SubCommand
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Replace a value on your environments
    Put {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
        #[arg(short, long)]
        project: String,
    },
    /// Add a new Key
    Add {
        #[arg(short, long)]
        project: String,
        key: String,
        value: String,
    },
    /// Remove a key
    RemoveKey {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        project: String,
    },
    /// Create a new Project
    CreateProject {
        name: String,
    },
    /// Remove a project
    RemoveProject {
        name: String,
    },
    /// Rename a project
    RenameProject {
        name: String,
        new_name: String,
    },
    /// Show all projects
    Projects,
    /// Get All keys
    Get {
        project: String,
        /// Supported format: json | dotenv | table 
        #[arg(short, long)]
        format: Option<String>,
    },
}

