extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate utils;

mod config;
mod container;
mod kafka_utils;
mod store;

use dotenv::dotenv;
use kafka_utils::kafka_handler::KafkaHandler;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use syrette::injectable;
use syrette::ptr::TransientPtr;
use syrette::DIContainer;
use utils::add;

use crate::{
    config::config::CONFIG,
    kafka_utils::kafka_handler::KafkaHandlerImpl,
    store::store::{Store, StoreImpl},
};

#[derive(thiserror::Error, Debug)]
enum DoSomethingError {
    #[error("Wrong answer")]
    WrongAnswer,
    #[error("A little bit more")]
    More,
    #[error("A little bit less")]
    Less,
}

fn do_something(input: &str) -> Result<String, DoSomethingError> {
    return match input {
        "a" => Err(DoSomethingError::WrongAnswer),
        "b" => Err(DoSomethingError::More),
        "c" => Err(DoSomethingError::Less),
        _ => Ok("ok".to_string()),
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let container = container::container::create_default_container();
    // let kafka_handler = container.get::<dyn KafkaHandler>()?.transient()?;

    let result = do_something("hello");
    match result {
        Ok(value) => println!("value is {value}"),
        Err(DoSomethingError::WrongAnswer) => println!("Wrong answer"),
        Err(DoSomethingError::More) => println!("A little bit more"),
        Err(DoSomethingError::Less) => println!("A little bit less"),
    }

    dotenv().ok();
    let source = "rust";
    println!("source is {source}");

    utils::kafka::create_topics::create_topics(&CONFIG.kafka_brokers).await;

    let subscribe_thread = thread::spawn(move || {
        let container = container::container::create_default_container();
        let kafka_handler = container
            .get::<dyn KafkaHandler>()
            .unwrap()
            .transient()
            .unwrap();
        kafka_handler.start_consuming();
    });

    let result = add(1, 2);
    println!("result is {result}");

    subscribe_thread.join().unwrap();

    Ok(())
}
