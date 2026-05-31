use loco_rs::model::{ModelError, ModelResult};
pub use super::_entities::event_types::{ActiveModel, Entity, Model};
use super::_entities::event_types::Column;
use sea_orm::entity::prelude::*;
pub type EventTypes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn get_id_by_name<C: ConnectionTrait>(db: &C, name: &str) -> ModelResult<i64> {
        Entity::find()
            .filter(Column::CodeName.eq(name))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
            .map(|m| m.id)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
