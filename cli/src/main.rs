use std::env::home_dir;

use clap::Parser;
use cult::{connection::Connection};

use crate::commands::{CultCli, SubCommand};

pub mod commands;

#[tokio::main]
async fn main() {
    let dir = home_dir().unwrap();
    let db_path = format!("{}/.cult", dir.to_str().unwrap());

    let conn = Connection::new(db_path).await;

    let values = CultCli::parse();

    match values.command {
        SubCommand::Put { .. } => todo!(),
        SubCommand::CreateProject { name } => {
            conn.project.create(name).await;
        },
        SubCommand::Projects => {
            let projects = conn.project.find_all().await;
            dbg!(&projects);
            for project in projects {
                println!("{}", project.name);
            }
        },
        SubCommand::Show { project } => todo!(),
        SubCommand::Get { project, file } => todo!(),
    }
}
