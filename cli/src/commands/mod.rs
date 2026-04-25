use clap::{Parser, Subcommand};

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

pub mod projects;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CultCli {
    #[command(subcommand)]
    pub command: SubCommand
}

#[derive(Subcommand)]
pub enum SubCommand {
    Put {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
        #[arg(short, long)]
        project: String,
    },
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
    RemoveProject {
        name: String,
    },
    Projects,
    Show {
        project: String,
    },
    Get {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        file: Option<String>,
    }
}
