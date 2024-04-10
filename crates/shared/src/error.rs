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

#[derive(Debug)]
pub enum ErrorKind {
    InvalidParameter,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidParameter => write!(f, "invalid parameter"),
        }
    }
}

impl Serialize for ErrorKind {
    fn serialize<S>(&self, s: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        match self {
            ErrorKind::InvalidParameter => {
                s.serialize_unit_variant("ErrorKind", 2, "InvalidParameter")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<&ErrorKind> for rocket::http::Status {
    fn from(kind: &ErrorKind) -> Self {
        match kind {
            ErrorKind::InvalidParameter => Self::BadRequest,
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
    macro_rules! error_from {
        ($($from:path),*) => {
            $(
                impl From<$from> for Error {
                    fn from(e: $from) -> Self {
                        let kind = e.into();

                        Self {
                            kind,
                            message: None,
                        }
                    }
                }
            )*
        };
    }

    macro_rules! error_kind_from {
        ($($kind:ident => $from:path),*) => {
            $(
                impl From<$from> for ErrorKind {
                    fn from(e: $from) -> Self {
                        Self::$kind(e)
                    }
                }
            )*
        };
    }

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

    pub(super) use error_from;
    pub(super) use error_kind_from;
}
