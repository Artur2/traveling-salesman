use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=export.geojson");
    println!("cargo:rerun-if-changed=bus_stops_named.geojson");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("export.geojson");
    let buses_path = Path::new(&out_dir).join("bus_stops_named.geojson");

    println!("starting copy");
    fs::copy("export.geojson", dest_path).unwrap();
    fs::copy("bus_stops_named.geojson", buses_path).unwrap();

    println!(
        "cargo::warning=The output directory is for export.geojson is: {}",
        out_dir
    );
}
