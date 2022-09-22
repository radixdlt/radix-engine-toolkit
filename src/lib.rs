pub mod address;
pub mod error;
pub mod macros;
pub mod memory;
pub mod models;
pub mod requests;
pub mod traits;
pub mod utils;
pub mod validation;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
