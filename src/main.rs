extern crate actix_web;
extern crate bloom_filter;
extern crate fasthash;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;

use std::sync::Arc;
use std::sync::Mutex;

use actix_web::*;
use simplelog::{Config, LevelFilter, TermLogger};

use bloom_filter::{AppState, BloomFilter};

use crate::handlers::{check_keys, push_keys};

mod api;
mod handlers;


fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default())
        .expect("failed to initialize logger");

    let bloom_filter_ref =
        Arc::new(Mutex::new(BloomFilter::new()));

    server::new(move ||
        App::with_state(AppState { filter: bloom_filter_ref.clone() })
            .resource("/push", |r| r.method(http::Method::POST).with(push_keys))
            .resource("/check", |r| r.method(http::Method::POST).with(check_keys)))
        .bind("127.0.0.1:3333")
        .unwrap()
        .run();
}
