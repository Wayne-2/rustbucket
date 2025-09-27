use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]

pub enum Error{
    Loginfailed
}

impl IntoResponse for Error{
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "uhmm omo your server don crash ooo").into_response()
    }
}