use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod nodes;
pub mod graph;
pub mod path_resolver;
pub mod processing_algorithm;
pub mod types;