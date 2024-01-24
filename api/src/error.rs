use rocket::{response::Responder, serde::Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
    flat_kind: FlatErrorKind,
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
    DieselError(diesel::result::Error),
    R2D2Error(r2d2::Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::DieselError(e) => e.fmt(f),
            ErrorKind::R2D2Error(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_from {
    ($($from:path),*) => {
        $(
            impl From<$from> for Error {
                fn from(e: $from) -> Self {
                    let kind = e.into();

                    Self {
                        flat_kind: Into::<FlatErrorKind>::into(&kind),
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

            impl From<$from> for FlatErrorKind {
                fn from(_: $from) -> Self {
                    Self::$kind
                }
            }
        )*
    };
}

error_from!(diesel::result::Error, r2d2::Error);

error_kind_from!(
    DieselError => diesel::result::Error,
    R2D2Error => r2d2::Error
);

impl From<&ErrorKind> for rocket::http::Status {
    fn from(kind: &ErrorKind) -> Self {
        match kind {
            ErrorKind::DieselError(_) => Self::InternalServerError,
            ErrorKind::R2D2Error(_) => Self::InternalServerError,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            flat_kind: (&kind).into(),
            kind,
            message: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
enum FlatErrorKind {
    DieselError,
    R2D2Error,
}

impl From<&ErrorKind> for FlatErrorKind {
    fn from(kind: &ErrorKind) -> Self {
        match kind {
            ErrorKind::DieselError(_) => Self::DieselError,
            ErrorKind::R2D2Error(_) => Self::R2D2Error,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResponse {
    kind: FlatErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl From<&Error> for ErrorResponse {
    fn from(e: &Error) -> Self {
        Self {
            kind: e.flat_kind,
            error: e.message.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::Response::build()
            .status((&self.kind).into())
            .header(rocket::http::ContentType::JSON)
            .finalize();

        let body = serde_json::to_string(&ErrorResponse::from(&self)).unwrap();

        response.set_sized_body(body.len(), std::io::Cursor::new(body));

        Ok(response)
    }
}
