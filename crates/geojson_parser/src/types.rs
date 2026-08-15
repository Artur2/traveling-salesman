use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone)]
pub struct Route {
    pub from: Option<String>,
    pub to: Option<String>
}

#[derive(Deserialize)]
pub struct Feature {
    pub properties: Route,
    pub geometry: Geometry
}

#[derive(Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_definition: String,
    pub coordinates: Value,
}

#[derive(Deserialize)]
pub struct Export {
    pub features: Vec<Feature>,
}

pub struct ParsingEntry {
    pub from: String,
    pub to: String,
    pub kms: f64
}

pub type ParsingResult<T> = Result<T, String>;