use super::_entities::payload_types;
pub use super::_entities::payload_types::{ActiveModel, Entity, Model};
use loco_rs::prelude::ModelResult;
use sea_orm::entity::prelude::*;
use sea_orm::Order;
pub type PayloadTypes = Entity;

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
    #[must_use]
    pub fn validate(&self, value: &str) -> bool {
        match self.value_type_id {
            1 => value.parse::<i64>().is_ok(),
            2 => value.parse::<bool>().is_ok(),
            3 => true,
            _ => false,
        }
    }

    /// Finds `PayloadTypes` models by specified ids
    ///
    /// # Errors
    /// Database error
    pub async fn find_by_ids(db: &DatabaseConnection, pt_ids: &[i64]) -> ModelResult<Vec<Self>> {
        Ok(Entity::find()
            .filter(payload_types::Column::Id.is_in(pt_ids.iter().copied()))
            .order_by_id(Order::Asc)
            .all(db)
            .await?)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
