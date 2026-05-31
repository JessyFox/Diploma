pub use super::_entities::payloads::{ActiveModel, Entity, Model};
use loco_rs::prelude::{Validatable, Validate};
use sea_orm::entity::prelude::*;
use serde::Deserialize;

pub type Payloads = Entity;

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(max = 50, message = "Value cannot exceed 50 characters"))]
    pub value: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            value: self.value.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
