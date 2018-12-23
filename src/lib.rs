extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use crate::bloom_filter::AppState;
pub use crate::bloom_filter::BloomFilter;

mod bloom_filter;
mod api;
pub mod handlers;

