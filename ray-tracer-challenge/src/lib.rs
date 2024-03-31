use mimalloc::MiMalloc;

pub mod draw;
pub mod io;
pub mod math;
pub mod scene;
pub mod util;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
