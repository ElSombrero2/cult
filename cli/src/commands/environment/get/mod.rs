use cult::connection::Connection;
use crate::commands::environment::get::format::{enums::Format, formatter::Formatter};

pub mod format;

pub async fn exec(conn: Connection, project: String, format: Option<String>) {
    if let Some(project) = conn.project.find_one(project).await {
        let envs = conn.environment.find_by_project(project.id).await;
        let formatter = Formatter::new();
        
        if let Some(format) = format {
            return formatter.print(envs, Format::from(format));
        }
        formatter.print(envs, Format::Table);
    }
}
