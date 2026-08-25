use serde::Deserialize;
use crate::shared_types::Geometry;

#[derive(Deserialize)]
pub struct BusStopsExport {
    pub features: Vec<BusStopEntry>,
}

#[derive(Deserialize)]
pub struct Properties {
    pub name: String,
}

#[derive(Deserialize)]
pub struct BusStopEntry {
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Default)]
pub struct BusStop {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}
