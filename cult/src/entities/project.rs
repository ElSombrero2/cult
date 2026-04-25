use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait};
use chrono::NaiveDateTime;
use crate::entities::environment;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  #[sea_orm(unique)]
  pub name: String,
  #[sea_orm(has_many)]
  pub environments: HasMany<environment::Entity>,
  #[sea_orm(default_expr = "Expr::current_timestamp()", nullable)]
  pub created_at: NaiveDateTime,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _: &C, insert: bool) -> Result<Self, DbErr> 
    where C: ConnectionTrait {
        println!("Before Save");
        if !insert {
           
        }
        Ok(self)
    }
}
