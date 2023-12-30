use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Inner,
}

/// Implements conversion from a string slice (`&str`) to an 
/// `ErrorResponse`. This allows to create an `ErrorResponse` 
/// from a string slice. The string slice is converted to 
/// an owned `String`, wrapped in a vector, and then wrapped 
/// in an `Inner` struct to match the structure of `ErrorResponse`.
///
/// # Arguments
///
/// * `msg` - A string slice that represents the error message.
///
/// # Returns
///
/// An instance of `ErrorResponse` where `errors.body` is a 
/// vector containing 
/// one string: the error message.
///
/// # Examples
///
/// ```
/// let error = ErrorResponse::from("This is an error message");
/// assert_eq!(error.errors.body[0], "This is an error message");
/// ```
impl std::convert::From<&str> for ErrorResponse {
    fn from(msg: &str) -> Self {
        Self {
            errors: Inner {
                body: vec![msg.to_owned()],
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Inner {
    pub body: Vec<String>,
}