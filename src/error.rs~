use std::fmt;


#[derive(Debug, PartialEq, Eq)]
pub enum RunnerError {
    WrongId,
    InitTask(String),
}

impl std::error::Error for RunnerError {}
impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongId => f.write_str("wrong Id"),
            Self::InitTask(s) => f.write_str(&s),
        }
    }
}


use actix_web::{http::{header::ContentType, StatusCode}, HttpResponse};
impl actix_web::error::ResponseError for RunnerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match self {
            Self::WrongId => StatusCode::BAD_REQUEST,
            Self::InitTask(_) => StatusCode::NOT_ACCEPTABLE,
        }
    }
}
