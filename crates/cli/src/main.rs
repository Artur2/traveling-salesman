use traveling_salesman_genetic::path_resolver::PathResolver;

fn main() {
    let mut path_resolver = PathResolver::new("test".to_owned());

    path_resolver.add_vertex("A".to_owned());
    path_resolver.add_vertex("B".to_owned());

    println!("Hello, world!");
}
