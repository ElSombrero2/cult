use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod project;

pub struct Connection {
  database: DatabaseConnection
}

impl Connection {
  pub async fn new (path: String) -> Self {
    let mut opt = ConnectOptions::new(path);

    opt.max_connections(1);

    let database = Database::connect(opt).await.unwrap();

    Connection { database }
  }
}
