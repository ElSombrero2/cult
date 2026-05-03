use cult::connection::Connection;

pub async fn exec(conn: Connection, project: String, key: String) {
   if let Some(project) = conn.project.find_one(project).await {
        conn.environment.remove(key, project.id).await;
   } 
}
