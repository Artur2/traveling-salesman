use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub(crate) mod nodes;
pub mod internal;
pub(crate) mod path_resolver;
