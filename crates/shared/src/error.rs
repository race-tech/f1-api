use r2d2::Error as R2D2Error;
use rocket::response::Responder;
use serde::ser::{Serialize, SerializeTupleVariant};

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

#[derive(Debug)]
pub enum ErrorKind {
    InvalidParameter,
    R2D2(String),
}

impl From<R2D2Error> for ErrorKind {
    fn from(e: R2D2Error) -> Self {
        ErrorKind::R2D2(e.to_string())
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        match self {
            InvalidParameter => write!(f, "invalid parameter"),
            R2D2(e) => e.fmt(f),
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
            R2D2(ref e) => {
                let mut sv = s.serialize_tuple_variant("ErrorKind", 1, "R2D2", 1)?;
                sv.serialize_field(e)?;
                sv.end()
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<&ErrorKind> for rocket::http::Status {
    fn from(kind: &ErrorKind) -> Self {
        use ErrorKind::*;

        match kind {
            InvalidParameter => Self::BadRequest,
            R2D2(_) => Self::InternalServerError,
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
        println!("{:?}", self);

        let mut response = rocket::Response::build()
            .status((&self.kind).into())
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
