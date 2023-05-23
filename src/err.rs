#[derive(Debug)]
pub struct AppletError
{
    pub status_code: i32,
    pub message: Option<String>
}

pub type AppletResult = Result<(), AppletError>;

impl AppletError
{
    pub fn new(status_code: i32, message: String) -> AppletError
    {
        AppletError { status_code: status_code, message: Some(message) }
    }

    pub fn new_nomsg(status_code: i32) -> AppletError
    {
        AppletError { status_code: status_code, message: None }
    }
}
