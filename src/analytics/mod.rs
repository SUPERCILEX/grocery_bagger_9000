#[cfg(not(target_arch = "wasm32"))]
pub use desktop::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(target_arch = "wasm32")]
mod web;
