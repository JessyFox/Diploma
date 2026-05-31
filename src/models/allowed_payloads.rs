use super::_entities::allowed_payloads;
pub use super::_entities::allowed_payloads::{ActiveModel, Entity, Model};
use loco_rs::model::ModelResult;
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use std::collections::HashSet;
pub type AllowedPayloads = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    /// Checks if specified payload types
    /// is allowed for specified event type
    ///
    /// # Errors
    /// Database error
    pub async fn is_allowed(
        db: &DatabaseConnection,
        event_type_id: i64,
        payload_type_ids: &[i64],
    ) -> ModelResult<bool> {
        let allowed: HashSet<i64> = Self::find()
            .select_only()
            .column(allowed_payloads::Column::PayloadTypeId)
            .filter(allowed_payloads::Column::EventTypeId.eq(event_type_id))
            .into_tuple::<(i64,)>()
            .all(db)
            .await?
            .into_iter()
            .map(|t| t.0)
            .collect();

        let requested: HashSet<i64> = payload_type_ids.iter().copied().collect();

        Ok(allowed == requested)
    }
}
