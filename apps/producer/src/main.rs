extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate utils;

mod config;
mod kafka_utils;

use crate::config::config::CONFIG;
use dotenv::dotenv;
use std::thread;
use utils::add;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let source = "rust";
    println!("source is {source}");

    utils::kafka::create_topics::create_topics(&CONFIG.kafka_brokers).await;

    let produce_purchase_events_thread = thread::spawn(move || {
        kafka_utils::produce::start_producing_purchase_events();
    });

    let produce_page_view_events_thread = thread::spawn(move || {
        kafka_utils::produce::start_producing_page_view_events();
    });

    let result = add(1, 2);
    println!("result is {result}");

    produce_purchase_events_thread.join().unwrap();
    produce_page_view_events_thread.join().unwrap();
}
