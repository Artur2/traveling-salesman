use clap::Parser;
use traveling_salesman_genetic::path_resolver::PathResolver;

type GeoJSONParser = geojson_parser::parser::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    geo_json_path: String,
    #[arg(short, long)]
    source: String,
    #[arg(short, long)]
    destination: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let parser = GeoJSONParser::new();
    let mut path_resolver = PathResolver::new("Test".to_owned());

    let parse_result = parser.parse(args.geo_json_path.as_str())?;

    parse_result.iter().for_each(|result| {
        if !path_resolver.has_vertex(result.to.as_str()) {
            path_resolver.add_vertex(result.to.to_owned());
        }
        if !path_resolver.has_vertex(result.from.as_str()) {
            path_resolver.add_vertex(result.from.to_owned());
        }
        if !path_resolver.has_connection(&result.from, &result.to) {
            path_resolver.connect_vertices(result.from.to_owned(), result.to.to_owned(), 10); // TODO: Retrieve route range in parser
        }
    });

    let found_path = path_resolver.resolve_optimal_path(
        args.source.as_str(),
        args.destination.as_str(),
        1000,
        1000,
        50,
    );

    found_path.iter().for_each(|result| println!("{}", result));

    Ok(())
}
