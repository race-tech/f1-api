use rocket::response::Responder;
use serde::ser::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
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
    R2D2,
    Mysql,
    EntityNotFound,
    IpHeaderNotFound,
    RateLimitReached,
    InternalServer,
    InternalRessource,
    Redis,
    MissingEnvVar,
    ConnectionPool,
    ParseInt,
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error {
            kind: ErrorKind::R2D2,
            message: Some(e.to_string()),
        }
    }
}

impl From<mysql::Error> for Error {
    fn from(e: mysql::Error) -> Self {
        Error {
            kind: ErrorKind::Mysql,
            message: Some(e.to_string()),
        }
    }
}

impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Self {
        Error {
            kind: ErrorKind::Redis,
            message: Some(e.to_string()),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        match self {
            InvalidParameter => write!(f, "invalid parameter"),
            R2D2 => write!(f, "r2d2 error"),
            Mysql => write!(f, "mysql error"),
            EntityNotFound => write!(f, "entity not found"),
            IpHeaderNotFound => write!(f, "ip header not found"),
            RateLimitReached => write!(f, "rate limit reached on given the sliding window"),
            InternalServer => write!(f, "an unexpected error occured"),
            InternalRessource => write!(f, "internal ressource queried by external user"),
            Redis => write!(f, "redis error"),
            MissingEnvVar => write!(f, "an environment variable is missing"),
            ConnectionPool => write!(f, "an error occured while setting-up a connection pool"),
            ParseInt => write!(f, "can't parse int"),
        }
    }
}

impl Serialize for ErrorKind {
    fn serialize<S>(&self, s: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        use ErrorKind::*;

        match self {
            InvalidParameter => s.serialize_unit_variant("ErrorKind", 0, "InvalidParameter"),
            R2D2 => s.serialize_unit_variant("ErrorKind", 1, "R2D2"),
            Mysql => s.serialize_unit_variant("ErrorKind", 2, "Mysql"),
            EntityNotFound => s.serialize_unit_variant("ErrorKind", 3, "EntityNotFound"),
            IpHeaderNotFound => s.serialize_unit_variant("ErrorKind", 4, "IpHeaderNotFound"),
            RateLimitReached => s.serialize_unit_variant("ErrorKind", 5, "RateLimitReached"),
            InternalServer => s.serialize_unit_variant("ErrorKind", 6, "InternalServerError"),
            InternalRessource => s.serialize_unit_variant("ErrorKind", 7, "InternalRessource"),
            Redis => s.serialize_unit_variant("ErrorKind", 8, "Redis"),
            MissingEnvVar => s.serialize_unit_variant("ErrorKind", 9, "MissingEnvVar"),
            ConnectionPool => s.serialize_unit_variant("ErrorKind", 10, "ConnectionPool"),
            ParseInt => s.serialize_unit_variant("ErrorKind", 11, "ParseInt"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for rocket::http::Status {
    fn from(kind: ErrorKind) -> Self {
        use ErrorKind::*;

        match kind {
            InvalidParameter => Self::BadRequest,
            R2D2 => Self::InternalServerError,
            Mysql => Self::InternalServerError,
            EntityNotFound => Self::NotFound,
            IpHeaderNotFound => Self::BadRequest,
            RateLimitReached => Self::TooManyRequests,
            InternalServer => Self::InternalServerError,
            InternalRessource => Self::Unauthorized,
            Redis => Self::InternalServerError,
            MissingEnvVar => Self::InternalServerError,
            ConnectionPool => Self::InternalServerError,
            ParseInt => Self::InternalServerError,
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

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::Response::build()
            .status((self.kind).into())
            .header(rocket::http::ContentType::JSON)
            .finalize();

        let body = serde_json::to_string(&ErrorResponse::from(self)).unwrap();

        response.set_sized_body(body.len(), std::io::Cursor::new(body));

        Ok(response)
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
}
