use std::any::Any;
use allo_isolate::IntoDart;

/// Errors that occur from normal code execution.
pub enum Error {
    /// Errors that implement [IntoDart].
    CustomError(Box<dyn BoxIntoDart>),
    /// Exceptional errors from panicking.
    Panic(Box<dyn Any + Send>),
}

impl Error {
    /// The message of the error.
    pub fn message(&self) -> String {
        match self {
            Error::CustomError(_e) => "Box<dyn BoxIntoDart>".to_string(),
            Error::Panic(panic_err) => error_to_string(panic_err),
        }
    }
}

impl IntoDart for Error {
    fn into_dart(self) -> DartAbi {
        match self {
            Error::CustomError(e) => e.box_into_dart(),
            Error::Panic(panic_err) => error_to_string(&panic_err).into_dart(),
        }
    }
}

fn error_to_string(panic_err: &Box<dyn Any + Send>) -> String {
    match panic_err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match panic_err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    }
        .to_string()
}

