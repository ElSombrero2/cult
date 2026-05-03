use cult::connection::Connection;

pub async fn exec(conn: Connection, name: String, new_name: String) {
    conn.project.rename(name, new_name).await;
}
