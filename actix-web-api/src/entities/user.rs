use sea_orm::entity::prelude::*;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};

/// User entity for SeaORM
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(column_type = "String(StringLen::N(255))", unique)]
    pub email: String,
    
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Convert SeaORM model to domain model
impl From<Model> for crate::models::User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Convert domain model to SeaORM ActiveModel for inserts
impl From<crate::models::User> for ActiveModel {
    fn from(user: crate::models::User) -> Self {
        Self {
            id: Set(user.id),
            email: Set(user.email),
            name: Set(user.name),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
    }
}