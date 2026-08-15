use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Route {
    pub from: Option<String>,
    pub to: Option<String>
}

#[derive(Deserialize)]
pub struct Feature {
    pub properties: Route
}

#[derive(Deserialize)]
pub struct Export {
    pub features: Vec<Feature>,
}

pub struct ParsingEntry {
    pub from: String,
    pub to: String
}

pub type ParsingResult<T> = Result<T, String>;