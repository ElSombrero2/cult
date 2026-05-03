use cult::connection::Connection;

pub async fn exec (conn: Connection, project: String, key: String, value: String) {
    if let Some(project) = conn.project.find_one(project).await {
        if conn.environment.add(key.clone(), value.clone(), project.id).await.is_some() {
           return println!("✔️ {key}={value} inserted!"); 
        }
    }
   println!("❌An error occured, please check if the project exists!"); 
}
