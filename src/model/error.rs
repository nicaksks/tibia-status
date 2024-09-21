use serde::Serialize;
#[derive(Serialize)]
pub struct Error {
    pub code: i16,
    pub message: String
}