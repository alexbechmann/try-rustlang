extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde_json;

mod config;
mod kafka_handler;
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
        kafka_handler::produce::produce();
    });

    let subscribe_thread = thread::spawn(move || {
        kafka_handler::subscribe::subscribe();
    });

    let result = add(1, 2);
    println!("result is {result}");

    subscribe_thread.join().unwrap();
    produce_thread.join().unwrap();
}
