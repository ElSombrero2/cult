use cult::connection::Connection;

pub async fn exec (conn: Connection, name: String) {
    if let Some(_) = conn.project.create(name).await {
        return println!("✅ Your project was created with success");
    }
    println!("❌ An error occurred during insertion; please check that the name is not already assigned to an existing project!");
}
