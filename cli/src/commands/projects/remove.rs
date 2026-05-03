use cult::connection::Connection;

pub async fn exec(conn: Connection, name: String) {
    conn.project.remove(name).await;
}
