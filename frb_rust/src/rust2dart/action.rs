use crate::generalized_isolate::IntoDart;
use crate::handler::error_handler::Error;
use crate::platform_types::DartAbi;

#[derive(Debug)]
pub enum Rust2DartAction {
    Success = 0,
    Error = 1,
    CloseStream = 2,
    Panic = 3,
}

impl From<&Error> for Rust2DartAction {
    fn from(value: &Error) -> Self {
        match value {
            Error::CustomError(_) => Self::Error,
            Error::Panic(_) => Self::Panic,
        }
    }
}

impl IntoDart for Rust2DartAction {
    fn into_dart(self) -> DartAbi {
        (self as i32).into_dart()
    }
}
