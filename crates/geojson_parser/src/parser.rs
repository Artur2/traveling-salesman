use crate::bus_stops_types::{BusStop, BusStopsExport};
use crate::route_types::{Export as RoutesExport, Feature, ParsingEntry};
use crate::shared_types::{ParsingResult, ParsingResultError};
use rayon::prelude::*;
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

    pub fn parse(
        &self,
        routes_file_path: &str,
        buses_file_path: &str,
    ) -> ParsingResult<Vec<ParsingEntry>> {
        let path = Path::new(routes_file_path);
        let bus_path = Path::new(buses_file_path);
        if !path.exists() || !bus_path.exists() {
            return Err(ParsingResultError::FileNotFound {
                path: path.to_str().unwrap().to_string(),
            });
        }

        let mut content = File::open(path).map_err(|e| ParsingResultError::ReadError {
            message: e.to_string(),
        })?;
        let mut buses_content =
            File::open(bus_path).map_err(|e| ParsingResultError::ReadError {
                message: e.to_string(),
            })?;
        let mut string = String::new();
        let mut buses_string = String::new();

        content
            .read_to_string(&mut string)
            .map_err(|e| ParsingResultError::ReadError {
                message: e.to_string(),
            })?;

        buses_content
            .read_to_string(&mut buses_string)
            .map_err(|e| ParsingResultError::ReadError {
                message: e.to_string(),
            })?;

        let parsed_routes: RoutesExport =
            serde_json::from_str(&string).map_err(|e| ParsingResultError::DeserializationError)?;
        let parsed_bus_stops: BusStopsExport = serde_json::from_str(&buses_string)
            .map_err(|e| ParsingResultError::DeserializationError)?;

        let bus_stops: Vec<BusStop> = parsed_bus_stops
            .features
            .iter()
            .map(|f| BusStop {
                name: f.properties.name.to_string(),
                latitude: f.geometry.coordinates[1].as_f64().unwrap(),
                longitude: f.geometry.coordinates[0].as_f64().unwrap(),
            })
            .collect();

        let routes = parsed_routes
            .features
            .iter()
            .filter(|f| f.properties.from.is_some() && f.properties.to.is_some())
            .map(|f| {
                let from = f.properties.from.as_ref().unwrap().clone();
                let to = f.properties.to.as_ref().unwrap().clone();
                let kms = self.calculate_kms(f, &bus_stops);
                ParsingEntry { from, to, kms }
            })
            .collect::<Vec<ParsingEntry>>();

        Ok(routes)
    }

    fn calculate_kms(&self, feature: &Feature, bus_stops: &Vec<BusStop>) -> f64 {
        let definition = feature.geometry.type_definition.clone();
        match definition.as_str() {
            "LineString" => {
                let mut current_lat: Option<f64> = None;
                let mut current_lon: Option<f64> = None;

                let mut distance = 0f64;

                for items in feature.geometry.coordinates.as_array() {
                    items.iter().for_each(|item| match item {
                        Value::Array(pairs) => {
                            let longitude = &pairs[0];
                            let latitude = &pairs[1];

                            if let (Some(lat), Some(lon)) = (latitude.as_f64(), longitude.as_f64())
                            {
                                let near_bus = self.get_near_bus_stop(lat, lon, bus_stops);

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
                                    let longitude = &pairs[0];
                                    let latitude = &pairs[1];

                                    if let (Some(lat), Some(lon)) =
                                        (latitude.as_f64(), longitude.as_f64())
                                    {
                                        let near_bus = self.get_near_bus_stop(lat, lon, bus_stops);

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

    fn get_near_bus_stop<'a>(
        &self,
        lat: f64,
        lon: f64,
        bus_stops: &'a Vec<BusStop>,
    ) -> ParsingResult<Option<&'a BusStop>> {
        let found_near_bus = bus_stops
            .par_iter()
            .enumerate()
            .map(|(i, bus_stop)| {
                let distance =
                    self.calculate_distance_in_km(bus_stop.latitude, bus_stop.longitude, lat, lon);
                (i, distance)
            })
            .reduce(
                || (0, f64::MAX),
                |a, b| {
                    if a.1 < b.1 { a } else { b }
                },
            );

        if found_near_bus.1 > 0.1f64 || found_near_bus.0 > bus_stops.len() {
            return Ok(None);
        }

        let bus_by_index = &bus_stops[found_near_bus.0];
        Ok(Some(bus_by_index))
    }

    fn calculate_distance_in_km(
        &self,
        current_lat: f64,
        current_lon: f64,
        next_lat: f64,
        next_lon: f64,
    ) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        let dlat = self.to_radians(next_lat - current_lat);
        let dlon = self.to_radians(next_lon - current_lon);

        // TODO: Рассчитывать изначально, что бы не в параллель рассчитывать
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

        EARTH_RADIUS_KM * c
    }

    #[inline(always)]
    fn to_radians(&self, angle: f64) -> f64 {
        PI / 180.0 * angle
    }
}

mod tests {
    use super::*;

    #[test]
    #[ignore = "Only for local run"]
    pub fn should_parse_file() -> ParsingResult<()> {
        let file = Path::new(env!("OUT_DIR")).join("export.geojson");

        let buses_file = Path::new(env!("OUT_DIR")).join("bus_stops_named.geojson");
        let parser = Parser;

        let result = parser.parse(file.to_str().unwrap(), buses_file.to_str().unwrap())?;

        assert!(result.len() > 0);
        Ok(())
    }
}
