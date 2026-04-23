use std::env::home_dir;

use cult::{connection::Connection};

#[tokio::main]
async fn main() {
  let dir = home_dir().unwrap();
  let db_path = format!("sqlite://{}/.cult/cult.data?mode=rwc", dir.to_str().unwrap());

  let conn= Connection::new(db_path).await;
}
