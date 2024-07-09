use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::ser::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}", message),
            None => self.kind.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    InvalidParameter,
    EntityNotFound,
    ResourceNotFound,
    IpHeaderNotFound,

    RateLimitReached,
    InternalServer,
    MissingEnvVar,
    ConnectionPool,

    // Std errors
    ParseInt,
    FromUtf8,

    // External error kinds
    BB8,
    Surreal,
    Figment,
    Serde,
    Axum,
}

macros::error_from!(Surreal => surrealdb::Error);
macros::error_from!(Figment => figment::Error);
macros::error_from!(Serde => serde_json::Error);
macros::error_from!(Axum => axum::Error);
macros::error_from!(Axum => axum::http::Error);
macros::error_from!(FromUtf8 => std::string::FromUtf8Error);

impl<E> From<bb8::RunError<E>> for Error
where
    E: std::error::Error + 'static,
{
    fn from(value: bb8::RunError<E>) -> Self {
        Self {
            kind: ErrorKind::BB8,
            message: Some(value.to_string()),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        match self {
            InvalidParameter => write!(f, "invalid parameter"),
            EntityNotFound => write!(f, "entity not found"),
            IpHeaderNotFound => write!(f, "ip header not found"),
            RateLimitReached => write!(f, "rate limit reached on given the sliding window"),
            InternalServer => write!(f, "an unexpected error occured"),
            ResourceNotFound => write!(f, "the queried resource was not found"),
            MissingEnvVar => write!(f, "an environment variable is missing"),
            ConnectionPool => write!(f, "an error occured while setting-up a connection pool"),

            ParseInt => write!(f, "can't parse int"),
            FromUtf8 => write!(f, "invalid utf8 string"),

            BB8 => write!(f, "bb8 error"),
            Surreal => write!(f, "surreal error"),
            Figment => write!(f, "figment error"),
            Serde => write!(f, "serde error"),
            Axum => write!(f, "axum error"),
        }
    }
}

impl Serialize for ErrorKind {
    fn serialize<S>(&self, s: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use ErrorKind::*;

        match self {
            InvalidParameter => s.serialize_unit_variant("ErrorKind", 0, "InvalidParameter"),
            EntityNotFound => s.serialize_unit_variant("ErrorKind", 1, "EntityNotFound"),
            IpHeaderNotFound => s.serialize_unit_variant("ErrorKind", 2, "IpHeaderNotFound"),
            RateLimitReached => s.serialize_unit_variant("ErrorKind", 3, "RateLimitReached"),
            InternalServer => s.serialize_unit_variant("ErrorKind", 4, "InternalServerError"),
            ResourceNotFound => s.serialize_unit_variant("ErrorKind", 5, "ResourceNotFound"),
            MissingEnvVar => s.serialize_unit_variant("ErrorKind", 6, "MissingEnvVar"),
            ConnectionPool => s.serialize_unit_variant("ErrorKind", 7, "ConnectionPool"),

            ParseInt => s.serialize_unit_variant("ErrorKind", 8, "ParseInt"),
            FromUtf8 => s.serialize_unit_variant("ErrorKind", 9, "FromUtf8"),

            Surreal => s.serialize_unit_variant("ErrorKind", 10, "Surreal"),
            Figment => s.serialize_unit_variant("ErrorKind", 13, "Figment"),
            Serde => s.serialize_unit_variant("ErrorKind", 14, "Serde"),
            Axum => s.serialize_unit_variant("ErrorKind", 15, "Axum"),
            BB8 => s.serialize_unit_variant("ErrorKind", 16, "BB8"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for StatusCode {
    fn from(kind: ErrorKind) -> Self {
        use ErrorKind::*;

        match kind {
            InvalidParameter => Self::BAD_REQUEST,
            EntityNotFound => Self::NOT_FOUND,
            IpHeaderNotFound => Self::BAD_REQUEST,
            RateLimitReached => Self::TOO_MANY_REQUESTS,
            InternalServer => Self::INTERNAL_SERVER_ERROR,
            ResourceNotFound => Self::NOT_FOUND,
            MissingEnvVar => Self::INTERNAL_SERVER_ERROR,
            ConnectionPool => Self::INTERNAL_SERVER_ERROR,

            ParseInt => Self::INTERNAL_SERVER_ERROR,
            FromUtf8 => Self::INTERNAL_SERVER_ERROR,

            Surreal => Self::INTERNAL_SERVER_ERROR,
            Figment => Self::INTERNAL_SERVER_ERROR,
            Serde => Self::INTERNAL_SERVER_ERROR,
            Axum => Self::INTERNAL_SERVER_ERROR,
            BB8 => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    kind: ErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        Self {
            kind: e.kind,
            error: e.message.clone(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status: StatusCode = self.kind.into();
        let body = serde_json::to_string(&ErrorResponse::from(self)).unwrap();

        (status, body).into_response()
    }
}

mod macros {
    #[macro_export]
    macro_rules! error {
        ($kind:ident) => {
            $crate::error::Error {
                kind: $crate::error::ErrorKind::$kind,
                message: None,
            }
        };
        ($kind:ident => $string:ident) => {
            $crate::error::Error {
                kind: $crate::error::ErrorKind::$kind,
                message: Some($string.to_string()),
            }
        };
        ($kind:ident => $($tt:tt)*) => {
            $crate::error::Error {
                kind: $crate::error::ErrorKind::$kind,
                message: Some(format!($($tt)*)),
            }
        };
    }

    macro_rules! error_from {
        ($kind:ident => $error:ty) => {
            impl From<$error> for Error {
                fn from(e: $error) -> Self {
                    Error {
                        kind: ErrorKind::$kind,
                        message: Some(e.to_string()),
                    }
                }
            }
        };
    }

    pub(super) use error_from;
}
