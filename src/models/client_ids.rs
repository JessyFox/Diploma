use super::_entities::client_ids;
pub use super::_entities::client_ids::{ActiveModel, Entity, Model};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
pub type ClientIds = Entity;

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
    /// Finds `ClientIds` model for specified ident
    ///
    /// # Errors
    /// Database error
    pub async fn find_by_uuid(db: &DatabaseConnection, ident: Uuid) -> ModelResult<Self> {
        Entity::find()
            .filter(client_ids::Column::Ident.eq(ident))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    /// Creates new `ClientIds` `ActiveModel`
    /// with new v4 `Uuid` ident
    ///
    /// # Errors
    /// Database error
    pub async fn create_new(db: &DatabaseConnection) -> ModelResult<Model> {
        Ok(Self {
            ident: ActiveValue::Set(uuid::Uuid::new_v4()),
            ..Default::default()
        }
        .insert(db)
        .await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
