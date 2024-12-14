use derive_getters::Getters;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Getters)]
pub struct ErrorResponse {
    method: String,
    env: String,
    error: ErrorResponseError,
}

#[derive(Deserialize, Getters)]
pub struct ErrorResponseError {
    code: isize,
    message: String,
}
