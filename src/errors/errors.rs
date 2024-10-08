// errors.rs

use std::fmt;
use std::io;
use std::sync::{MutexGuard, PoisonError};

#[derive(Debug)]
pub enum InterpreterError {
    IoError(io::Error),
    ParseError(String),
    ThreadError(String),
    LockError(String),
    FileNotFound(String),
    FileReadError(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::FileNotFound(msg) => write!(f, "File Not Found: {}", msg),
            InterpreterError::FileReadError(msg) => write!(f, "File Read Error: {}", msg),
            InterpreterError::IoError(e) => write!(f, "I/O Error: {}", e),
            InterpreterError::ParseError(s) => write!(f, "Parse Error: {}", s),
            InterpreterError::ThreadError(s) => write!(f, "Thread Error: {}", s),
            InterpreterError::LockError(s) => write!(f, "Lock Error: {}", s),
        }
    }
}

impl From<io::Error> for InterpreterError {
    fn from(error: io::Error) -> Self {
        InterpreterError::IoError(error)
    }
}

// Removed the specific implementation for Vec<Vec<char>>

// Retain the generic implementation
impl<T> From<PoisonError<MutexGuard<'_, T>>> for InterpreterError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        InterpreterError::LockError("Failed to lock Mutex".to_string())
    }
}
