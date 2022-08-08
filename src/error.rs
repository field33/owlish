use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<iref::Error> for Error {
    fn from(e: iref::Error) -> Self {
        Error {
            message: e.to_string(),
        }
    }
}
