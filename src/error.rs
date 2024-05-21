use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

pub enum ErrorKind { IO, Readline }

pub enum Error {
    Root { message: String },
    Child { message: String, source: Box<Error> },
    Imported { kind: ErrorKind, source: Box<dyn std::error::Error> },
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::IO => "I/O",
            ErrorKind::Readline => "Readline"
        }
    }
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Root { message } => { write!(f, "{}", message) }
            Error::Child { message, source } =>
                { write!(f, "{message}: {source}") }
            Error::Imported { kind, source } =>
                { write!(f, "{kind}: {source}") }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Root { .. } => None,
            Error::Child { source, .. } => Some(source.as_ref()),
            Error::Imported { source, .. } => Some(source.as_ref()),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Root { message }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Root { message: message.to_string() }
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        import_error(ErrorKind::IO, io_error)
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(readline_error: rustyline::error::ReadlineError) -> Self {
        import_error(ErrorKind::Readline, readline_error)
    }
}

fn import_error<E: std::error::Error + 'static>(kind: ErrorKind, source: E) -> Error {
    Error::Imported { kind, source: Box::new(source) }
}

fn wrap_error<M: Borrow<str>>(message: M, source: Error) -> Error {
    Error::Child { message: message.borrow().to_string(), source: Box::new(source) }
}

pub(crate) fn for_file<T, F: Borrow<str>>(file: F, result: Result<T, std::io::Error>)
    -> Result<T, Error> {
    result.map_err(|io_error| {
        let message = file.borrow().to_string();
        let source = Error::from(io_error);
        wrap_error(message, source)
    })
}