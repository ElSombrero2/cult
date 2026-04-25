use sea_orm::entity::prelude::*;

use crate::entities::project;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "environment")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  #[sea_orm(index)]
  pub key: String,
  pub value: String,
  pub project_id: i32,
  #[sea_orm(belongs_to, from = "project_id", to = "id")]
  pub project: HasOne<project::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
