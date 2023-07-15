extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
#[macro_use]
extern crate fstrings;

mod config;
mod kafka_utils;
mod message;
mod utils;

use std::thread;

use dotenv::dotenv;
use utils::add::add;

fn main() {
    dotenv().ok();
    let source = "rust";
    println!("source is {source}");

    let produce_thread = thread::spawn(move || {
        kafka_utils::produce::produce();
    });

    let subscribe_thread = thread::spawn(move || {
        kafka_utils::subscribe::subscribe();
    });

    let result = add(1, 2);
    println!("result is {result}");

    subscribe_thread.join().unwrap();
    produce_thread.join().unwrap();
}
