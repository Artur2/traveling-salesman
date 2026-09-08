use crate::shared_types::Geometry;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Feature {
    pub geometry: Geometry
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
