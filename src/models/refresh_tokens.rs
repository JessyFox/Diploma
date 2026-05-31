pub use super::_entities::refresh_tokens::{ActiveModel, Entity, Model};
use crate::models::_entities::refresh_tokens;
use loco_rs::{
    auth::jwt,
    prelude::{cookie::Cookie, *},
};
use sea_orm::entity::prelude::*;
use serde_json::Map;
use time::{Duration, OffsetDateTime};
pub type RefreshTokens = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        match (insert, self.updated_at.is_unchanged()) {
            (false, true) => {
                let mut this = self;
                this.updated_at = Set(chrono::Utc::now().into());
                Ok(this)
            }
            (true, _) => {
                let mut this = self;
                this.jti = Set(Uuid::new_v4());
                Ok(this)
            }
            _ => Ok(self),
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// Creates a refresh token
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_refresh_token<'a>(
        &self,
        secret: &'a str,
        expiration: u64,
        name: &'a str,
        pid: &'a Uuid,
    ) -> ModelResult<Cookie<'a>> {
        let mut map = Map::new();
        map.entry("jti")
            .or_insert(serde_json::Value::String(self.jti.to_string()));
        let token: String = jwt::JWT::new(secret)
            .generate_token(expiration, pid.to_string(), map)
            .map_err(ModelError::from)?;
        Ok(Cookie::build((name, token))
            .http_only(true)
            .same_site(cookie::SameSite::Strict)
            .secure(true)
            .path("/")
            .expires(OffsetDateTime::now_utc() + Duration::seconds(expiration.cast_signed()))
            .build())
    }

    /// Find refresh token in db by jti value
    ///
    /// # Errors
    /// When no entries found or database error
    pub async fn find_by_jti(db: &DatabaseConnection, jti: Uuid) -> ModelResult<Self> {
        let token = Entity::find()
            .filter(refresh_tokens::Column::Jti.eq(jti))
            .one(db)
            .await?;
        token.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    /// Revokes old refresh token and generates new
    ///
    /// # Errors
    ///
    /// when generated jti is not unique or database query error
    pub async fn rotate_refresh_token(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.clone().revoke_token(db).await?;
        self.id = ActiveValue::NotSet;
        self.jti = Set(Uuid::new_v4());
        self.insert(db).await.map_err(ModelError::from)
    }

    /// Revokes refresh token
    ///
    /// # Errors
    ///
    /// when database query error
    pub async fn revoke_token(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.revoked = Set(true);
        self.clone().update(db).await.map_err(ModelError::from)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
