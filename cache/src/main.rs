extern crate byteorder;
extern crate bytes;
extern crate core;
extern crate evmap;
extern crate int_hash;
extern crate lazy_static;

extern crate common;
extern crate server;


pub mod cache;
pub mod data;
pub mod logic;
pub mod app;

use server::cache::server::Server;

use app::app::CompleteCacheApp;
use cache::cache::Cache;

fn main() {
    println!("Hello, world!");

    let app = Box::new(CompleteCacheApp::new(Cache::new()));

    let server: Server = Server::new(app);

    Server::start_small_load_optimized(server, "0.0.0.0", 4321);
}
