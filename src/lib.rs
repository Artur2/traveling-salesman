use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub(crate) mod graph;
pub(crate) mod nodes;
pub mod internal;
