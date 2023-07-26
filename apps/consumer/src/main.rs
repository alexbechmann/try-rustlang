extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate shaku;
extern crate utils;
#[macro_use]
extern crate shaku_derive;

mod config;
mod kafka_utils;
mod store;

use dotenv::dotenv;
use kafka_utils::kafka_handler::KafkaHandler;
use shaku::{module, HasComponent};
use std::thread;
use utils::add;

use crate::{
    config::config::CONFIG,
    kafka_utils::kafka_handler::KafkaHandlerImpl,
    store::store::{Store, StoreImpl},
};

module! {
    AppModule {
        components = [StoreImpl, KafkaHandlerImpl],
        providers = []
    }
}

lazy_static! {
    static ref APP_MODULE: AppModule = {
        return AppModule::builder().build();
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let source = "rust";
    println!("source is {source}");

    utils::kafka::create_topics::create_topics(&CONFIG.kafka_brokers).await;

    let subscribe_thread = thread::spawn(move || {
        let kafka_handler: &dyn KafkaHandler = APP_MODULE.resolve_ref();
        kafka_handler.start_consuming();
    });

    let result = add(1, 2);
    println!("result is {result}");

    subscribe_thread.join().unwrap();
}
