use std::{fs, rc::Rc};

use sea_orm::{ConnectOptions, Database};

use crate::{connection::managers::{environment::EnvironmentManager, project::ProjectManager}, entities::{environment, project}};

pub mod managers;

pub struct Connection {
    pub project: ProjectManager,
    pub environment: EnvironmentManager,
}

impl Connection {
    pub async fn new(path: String) -> Self {
        let mut create_table = false;
        if !fs::exists(path.clone()).unwrap() {
            fs::create_dir(path.clone()).unwrap();
            create_table = true;
        }
        let mut opt = ConnectOptions::new(format!("sqlite://{path}/cult.data?mode=rwc"));

        opt.max_connections(1);

        let db = Database::connect(opt).await.unwrap();

        if create_table {
            db.get_schema_builder()
                .register(environment::Entity)
                .register(project::Entity)
                .apply(&db).await.unwrap();
        }

        let database = Rc::new(db);

        let project = ProjectManager::new(database.clone());
        let environment = EnvironmentManager::new(database.clone());

        Connection { project, environment }
    }
}
