use std::env::home_dir;

use clap::Parser;
use cult::connection::Connection;

use crate::commands::{environment, projects, CultCli, SubCommand};

pub mod commands;
pub mod utils;

#[tokio::main]
async fn main() {
    let dir = home_dir().unwrap();
    let db_path = format!("{}/.cult", dir.to_str().unwrap());

    let conn = Connection::new(db_path).await;

    let values = CultCli::parse();

    match values.command {
        SubCommand::Put { project, key, value } => environment::put::exec(conn, project, key, value).await,
        SubCommand::CreateProject { name } => projects::create::exec(conn, name).await,
        SubCommand::Add { key, value, project } => environment::add::exec(conn, project, key, value).await,
        SubCommand::Projects => projects::list::exec(conn).await,
        SubCommand::Get { project, format } => environment::get::exec(conn, project, format).await,
        SubCommand::RemoveKey { project, key } => environment::remove::exec(conn, project, key).await,
        SubCommand::RemoveProject { name } => projects::remove::exec(conn, name).await,
        SubCommand::RenameProject { name, new_name } => projects::rename::exec(conn, name, new_name).await,
    }
}
