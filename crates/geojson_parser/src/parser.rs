use crate::types::{Export, ParsingEntry, ParsingResult};
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

        let routes = parsed
            .features
            .iter()
            .filter(|f| f.properties.from.is_some() && f.properties.to.is_some())
            .map(|f| {
                let from = f.properties.from.as_ref().unwrap().clone();
                let to = f.properties.to.as_ref().unwrap().clone();
                ParsingEntry { from, to }
            })
            .collect::<Vec<ParsingEntry>>();

        Ok(routes)
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
