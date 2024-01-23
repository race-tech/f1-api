#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    MissingEnvVar,
    ConnectionPoolError,
}

#[macro_export]
macro_rules! error {
    ($kind:ident => $($msg:tt)*) => {
        $crate::error::Error {
            kind: $crate::error::ErrorKind::$kind,
            message: format!($($msg)*),
        }
    };
}
