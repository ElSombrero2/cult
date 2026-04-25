use std::env::home_dir;

use clap::Parser;
use cult::{connection::Connection};

use crate::commands::{CultCli, SubCommand};

pub mod commands;
pub mod utils;

#[tokio::main]
async fn main() {
    let dir = home_dir().unwrap();
    let db_path = format!("{}/.cult", dir.to_str().unwrap());

    let conn = Connection::new(db_path).await;

    let values = CultCli::parse();

    match values.command {
        SubCommand::Put { .. } => todo!(),
        SubCommand::CreateProject { name } => commands::projects::create::exec(conn, name).await,
        SubCommand::Projects => commands::projects::list::exec(conn).await,
        SubCommand::Show { .. } => todo!(),
        SubCommand::Get { .. } => todo!(),
        _ => todo!(),
    }
}
