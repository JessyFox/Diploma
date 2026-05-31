//! # Custom error type for application

use axum::response::IntoResponse;
use axum::{
    extract::rejection::JsonRejection,
    http::{
        header::{InvalidHeaderName, InvalidHeaderValue},
        method::InvalidMethod,
        StatusCode,
    },
};
use loco_rs::controller::Json;
use loco_rs::model::ModelError;
use loco_rs::prelude::Response;
use loco_rs::{controller::ErrorDetail, validation::ModelValidationErrors, Error};

impl From<serde_json::Error> for ApiError {
    fn from(val: serde_json::Error) -> Self {
        Self::JSON(val).bt()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{inner}\n{backtrace}")]
    WithBacktrace {
        inner: Box<Self>,
        backtrace: Box<std::backtrace::Backtrace>,
    },

    #[error("{0}")]
    Message(String),

    #[error(
        "error while running worker: no queue provider populated in context. Did you configure \
         BackgroundQueue and connection details in `queue` in your config file?"
    )]
    QueueProviderMissing,

    #[error("task not found: '{0}'")]
    TaskNotFound(String),

    #[error(transparent)]
    Scheduler(#[from] loco_rs::scheduler::Error),

    #[error(transparent)]
    Axum(#[from] axum::http::Error),

    #[error(transparent)]
    JSON(serde_json::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error("cannot parse `{1}`: {0}")]
    YAMLFile(#[source] serde_yaml::Error, String),

    #[error(transparent)]
    YAML(#[from] serde_yaml::Error),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error("Worker error: {0}")]
    Worker(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    DB(#[from] sea_orm::DbErr),

    #[error("{0}")]
    Hash(String),

    // API
    #[error("{0}")]
    Unauthorized(String),

    // API
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("Access is forbidden")]
    Forbidden,

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("")]
    CustomError(StatusCode, ErrorDetail),

    #[error("internal server error")]
    InternalServerError,

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error(transparent)]
    InvalidMethod(#[from] InvalidMethod),

    #[error(transparent)]
    TaskJoinError(#[from] tokio::task::JoinError),

    // Model
    #[error(transparent)]
    Model(#[from] ModelError),

    #[error(transparent)]
    Storage(#[from] loco_rs::storage::StorageError),

    #[error(transparent)]
    Cache(#[from] loco_rs::cache::CacheError),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Validation(#[from] ModelValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
}

impl ApiError {
    pub fn wrap(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Any(Box::new(err)) //.bt()
    }

    pub fn msg(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Message(err.to_string()) //.bt()
    }
    #[must_use]
    pub fn string(s: &str) -> Self {
        Self::Message(s.to_string())
    }
    #[must_use]
    pub fn bt(self) -> Self {
        let backtrace = std::backtrace::Backtrace::capture();
        match backtrace.status() {
            std::backtrace::BacktraceStatus::Disabled
            | std::backtrace::BacktraceStatus::Unsupported => self,
            _ => Self::WithBacktrace {
                inner: Box::new(self),
                backtrace: Box::new(backtrace),
            },
        }
    }
}

impl IntoResponse for ApiError {
    /// Convert an `Error` into an HTTP response.
    #[allow(clippy::cognitive_complexity)]
    fn into_response(self) -> Response {
        match &self {
            Self::WithBacktrace {
                inner,
                backtrace: _,
            } => {
                tracing::error!(
                error.msg = %inner,
                error.details = ?inner,
                "controller_error"
                );
            }
            err => {
                tracing::error!(
                error.msg = %err,
                error.details = ?err,
                "controller_error"
                );
            }
        }

        let public_facing_error = match self {
            Self::NotFound(res) => (
                StatusCode::NOT_FOUND,
                ErrorDetail::new("not_found", &format!("{} was not found", res)),
            ),
            Self::Unauthorized(err) => {
                tracing::warn!(err);
                (
                    StatusCode::UNAUTHORIZED,
                    ErrorDetail::new(
                        "unauthorized",
                        "You do not have permission to access this resource",
                    ),
                )
            }
            Self::CustomError(status_code, data) => (status_code, data),
            Self::BadRequest(err) => (
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Bad Request", &err),
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                ErrorDetail::new("Forbidden", "Access is forbidden"),
            ),
            Self::UnprocessableEntity(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDetail::new("Unprocessable entity", &err),
            ),
            Self::JsonRejection(err) => {
                tracing::debug!(err = err.body_text(), "json rejection");
                (err.status(), ErrorDetail::with_reason("Bad Request"))
            }
            Self::Validation(ref errors) => (
                StatusCode::BAD_REQUEST,
                ErrorDetail {
                    error: None,
                    description: None,
                    errors: Some(serde_json::to_value(&errors.errors).unwrap_or_default()),
                },
            ),
            Self::Model(e) => match e {
                ModelError::EntityNotFound => (
                    StatusCode::NOT_FOUND,
                    ErrorDetail::new("not_found", "Resource was not found"),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorDetail::new("internal_server_error", "Internal Server Error"),
                ),
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("internal_server_error", "Internal Server Error"),
            ),
        };

        (public_facing_error.0, Json(public_facing_error.1)).into_response()
    }
}

impl From<loco_rs::Error> for ApiError {
    fn from(value: Error) -> Self {
        match value {
            Error::WithBacktrace { inner, backtrace } => ApiError::WithBacktrace {
                backtrace,
                inner: Box::new((*inner).into()),
            },
            Error::Message(m) => ApiError::Message(m),
            Error::QueueProviderMissing => ApiError::QueueProviderMissing,
            Error::TaskNotFound(m) => ApiError::TaskNotFound(m),
            Error::Scheduler(e) => ApiError::Scheduler(e),
            Error::Axum(e) => ApiError::Axum(e),
            Error::Tera(_) => ApiError::InternalServerError,
            Error::JSON(e) => ApiError::JSON(e),
            Error::JsonRejection(m) => ApiError::JsonRejection(m),
            Error::YAMLFile(e, m) => ApiError::YAMLFile(e, m),
            Error::YAML(e) => ApiError::YAML(e),
            Error::EnvVar(e) => ApiError::EnvVar(e),
            Error::EmailSender(_) => ApiError::InternalServerError,
            Error::Smtp(_) => ApiError::InternalServerError,
            Error::Worker(m) => ApiError::Worker(m),
            Error::IO(e) => ApiError::IO(e),
            Error::DB(e) => ApiError::DB(e),
            Error::ParseAddress(_) => ApiError::InternalServerError,
            Error::Hash(m) => ApiError::Hash(m),
            Error::Unauthorized(m) => ApiError::Unauthorized(m),
            Error::NotFound => ApiError::NotFound("Resource".to_string()),
            Error::BadRequest(m) => ApiError::BadRequest(m),
            Error::CustomError(c, e) => ApiError::CustomError(c, e),
            Error::InternalServerError => ApiError::InternalServerError,
            Error::InvalidHeaderValue(e) => ApiError::InvalidHeaderValue(e),
            Error::InvalidHeaderName(e) => ApiError::InvalidHeaderName(e),
            Error::InvalidMethod(e) => ApiError::InvalidMethod(e),
            Error::TaskJoinError(e) => ApiError::TaskJoinError(e),
            Error::Model(e) => ApiError::Model(e),
            Error::Redis(_) => ApiError::InternalServerError,
            Error::Sqlx(_) => ApiError::InternalServerError,
            Error::Storage(e) => ApiError::Storage(e),
            Error::Cache(e) => ApiError::Cache(e),
            Error::Generators(_) => ApiError::InternalServerError,
            Error::VersionCheck(_) => ApiError::InternalServerError,
            Error::SemVer(_) => ApiError::InternalServerError,
            Error::Any(e) => ApiError::Any(e),
            Error::Validation(e) => ApiError::Validation(e),
            Error::AxumFormRejection(e) => ApiError::AxumFormRejection(e),
        }
    }
}
