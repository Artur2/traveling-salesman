use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_definition: String,
    pub coordinates: Value,
}