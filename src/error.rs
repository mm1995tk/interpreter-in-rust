use std::str::Utf8Error;

pub enum InterpreterError {
    Utf8Error(Utf8Error),
    WhiteSpaceError
}