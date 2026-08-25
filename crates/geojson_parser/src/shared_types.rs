use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_definition: String,
    pub coordinates: Value,
}

pub type ParsingResult<T> = Result<T, ParsingResultError>;

#[derive(Debug)]
pub enum ParsingResultError {
    Unknown,
    FileNotFound { path: String },
    ReadError { message: String },
    DeserializationError,
    NoBusStop
}