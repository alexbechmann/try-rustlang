#[macro_use]
extern crate fstrings;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate utils;

mod config;
mod kafka_utils;

use dotenv::dotenv;
use std::thread;
use utils::add;

fn main() {
    dotenv().ok();
    let source = "rust";
    println!("source is {source}");

    let produce_thread = thread::spawn(move || {
        kafka_utils::produce::produce();
    });

    let result = add(1, 2);
    println!("result is {result}");

    produce_thread.join().unwrap();
}
