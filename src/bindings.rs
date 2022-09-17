#[cfg(not(target_arch = "wasm32"))]
mod export;
#[cfg(target_arch = "wasm32")]
mod import;

#[cfg(not(target_arch = "wasm32"))]
pub use export::*;

#[cfg(target_arch = "wasm32")]
pub use import::*;
