use actix_web::error::Error as ActixWebError;
use std::convert::From;
// use std::fmt::{self, Debug, Display};
// use std::fmt;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("hoge error happen.")]
    HogeError(String),
    // err: anyhow::Error,
}

impl From<AppError> for ActixWebError {
    fn from(err: AppError) -> ActixWebError {
        match err {
            AppError::HogeError(_str) => actix_web::error::ErrorNotFound("not found error")
        }
    }
}

impl From<DieselError> for AppError {
    fn from(_error: DieselError) -> Self {
        AppError::HogeError("diesel error".to_string())
    }
}