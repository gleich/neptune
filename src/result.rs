use rocket::response::{Responder, self};

pub struct Error(anyhow::Error);
pub type Result<T = ()> = std::result::Result<T, Error>;

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        response::Debug(self.0).respond_to(request)
    }
}