use crate::types::{Export, Feature, ParsingEntry, ParsingResult, Route};
use serde_json::Value;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[must_use = "Main entry point for parsing geojson format"]
pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, path: &str) -> ParsingResult<Vec<ParsingEntry>> {
        let path = Path::new(path);
        if !Path::exists(path) {
            return Err(format!("{} does not exist", path.display()));
        }

        let mut content = File::open(path).map_err(|e| e.to_string())?;
        let mut string = String::new();

        content
            .read_to_string(&mut string)
            .map_err(|e| e.to_string())?;

        let parsed: Export = serde_json::from_str(&string).map_err(|e| e.to_string())?;

        let mut routes = parsed
            .features
            .iter()
            .filter(|f| f.properties.from.is_some() && f.properties.to.is_some())
            .map(|f| {
                let from = f.properties.from.as_ref().unwrap().clone();
                let to = f.properties.to.as_ref().unwrap().clone();
                let kms = self.calculate_kms(f);
                ParsingEntry { from, to, kms }
            })
            .collect::<Vec<ParsingEntry>>();

        parsed
            .features
            .iter()
            .filter(|f| f.properties.relations.is_some())
            .for_each(|f| {
                f.properties.relations.iter().for_each(|rel| {
                    rel.iter().for_each(|rel| {
                        if let (Some(from), Some(to)) = (&rel.reltags.from, &rel.reltags.to) {
                            routes.push(ParsingEntry {
                                from: from.clone(),
                                to: to.clone(),
                                kms: 0f64, // TODO: Add correct weight with rework of parsing geojson(need add points to lines with weights) bus stops will be vertices, 
                            })
                        }
                    })
                })
            });

        Ok(routes)
    }

    fn calculate_kms(&self, feature: &Feature) -> f64 {
        let definition = feature.geometry.type_definition.clone();
        match definition.as_str() {
            "LineString" => {
                let mut current_lat: Option<f64> = None;
                let mut current_lon: Option<f64> = None;

                let mut distance = 0f64;

                for items in feature.geometry.coordinates.as_array() {
                    items.iter().for_each(|item| match item {
                        Value::Array(pairs) => {
                            let latitude = &pairs[0];
                            let longitude = &pairs[1];

                            if let (Some(lat), Some(lon)) = (latitude.as_f64(), longitude.as_f64())
                            {
                                if current_lat.is_none() && current_lon.is_none() {
                                    current_lat = Some(lat);
                                    current_lon = Some(lon);
                                } else {
                                    let next_lat = lat;
                                    let next_lon = lon;

                                    let kms = self.calculate_distance_in_km(
                                        current_lat.unwrap(),
                                        current_lon.unwrap(),
                                        next_lat,
                                        next_lon,
                                    );
                                    distance += kms;

                                    current_lat = Some(next_lat);
                                    current_lon = Some(next_lon);
                                }
                            }
                        }
                        _ => panic!(),
                    });
                }

                distance
            }
            "MultiLineString" => {
                let mut current_lat: Option<f64> = None;
                let mut current_lon: Option<f64> = None;

                let mut distance = 0f64;

                for items in feature.geometry.coordinates.as_array() {
                    items.iter().for_each(|item| match item {
                        Value::Array(inner_array) => {
                            inner_array.iter().for_each(|array| match array {
                                Value::Array(pairs) => {
                                    let latitude = &pairs[0];
                                    let longitude = &pairs[1];

                                    if let (Some(lat), Some(lon)) =
                                        (latitude.as_f64(), longitude.as_f64())
                                    {
                                        if current_lat.is_none() && current_lon.is_none() {
                                            current_lat = Some(lat);
                                            current_lon = Some(lon);
                                        } else {
                                            let next_lat = lat;
                                            let next_lon = lon;

                                            let kms = self.calculate_distance_in_km(
                                                current_lat.unwrap(),
                                                current_lon.unwrap(),
                                                next_lat,
                                                next_lon,
                                            );
                                            distance += kms;

                                            current_lat = Some(next_lat);
                                            current_lon = Some(next_lon);
                                        }
                                    }
                                }
                                Value::Null => {}
                                _ => panic!(),
                            })
                        }
                        _ => panic!(),
                    })
                }

                distance
            }
            _ => 0f64,
        }
    }

    fn calculate_distance_in_km(
        &self,
        current_lat: f64,
        current_lon: f64,
        next_lat: f64,
        next_lon: f64,
    ) -> f64 {
        const earth_radius_km: f64 = 6371.0;
        let dlat = self.to_radians(next_lat - current_lat);
        let dlon = self.to_radians(next_lon - current_lon);

        let rlat = self.to_radians(current_lat);
        let rlat2 = self.to_radians(next_lat);

        let mut a = f64::sin(dlat / 2f64) * f64::sin(dlat / 2f64)
            + f64::sin(dlon / 2f64) * f64::sin(dlon / 2f64) * f64::cos(rlat) * f64::cos(rlat2);

        if a > 1.0 {
            a = 1.0
        }

        let mut inside_sqrt = 1.0 - a;
        if inside_sqrt < 0.0 {
            inside_sqrt = 0.0
        }

        let c = 2f64 * f64::atan2(f64::sqrt(a), f64::sqrt(inside_sqrt));

        earth_radius_km * c
    }

    fn to_radians(&self, angle: f64) -> f64 {
        PI / 180.0 * angle
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn should_parse_file() -> ParsingResult<()> {
        let file = "export.geojson";
        let parser = Parser;

        let result = parser.parse(file)?;

        assert!(result.len() > 0);
        Ok(())
    }
}
