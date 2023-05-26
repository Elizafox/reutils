#[derive(Debug)]
pub struct Error {
    pub code: i32,
    pub message: Option<String>,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

impl Error {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message: Some(message),
        }
    }

    pub fn new_nomsg(code: i32) -> Self {
        Self {
            code,
            message: None,
        }
    }
}
