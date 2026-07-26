use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub(crate) mod nodes;
pub(crate) mod internal;
pub mod path_resolver;
