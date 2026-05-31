use async_trait::async_trait;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use utoipa::ToSchema;
use uuid::Uuid;

pub use super::_entities::users::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginParams {
    pub login: String,
    pub password: String,
    pub scope: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub login: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            login: self.login.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            this.pid = Set(Uuid::new_v4());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_api_key(_db: &DatabaseConnection, _api_key: &str) -> ModelResult<Self> {
        Err(ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(query::condition().eq(users::Column::Email, email).build())
            .filter(users::Column::IsActive.eq(true))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided login
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_login(db: &DatabaseConnection, login: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(query::condition().eq(users::Column::Login, login).build())
            .filter(users::Column::IsActive.eq(true))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = Entity::find()
            .filter(
                query::condition()
                    .eq(users::Column::Pid, parse_uuid)
                    .build(),
            )
            .filter(users::Column::IsActive.eq(true))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: u64) -> ModelResult<String> {
        let mut map = Map::new();
        map.insert("id".to_string(), serde_json::Value::Number(self.id.into()));
        jwt::JWT::new(secret)
            .generate_token(expiration, self.pid.to_string(), map)
            .map_err(ModelError::from)
    }
}

impl ActiveModel {
    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.update(db).await.map_err(ModelError::from)
    }

    /// Deactivates user
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn deactivate(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.is_active = Set(false);
        self.update(db).await.map_err(ModelError::from)
    }
}
