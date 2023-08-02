extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate shaku;
extern crate shaku_derive;
extern crate utils;

mod config;
mod kafka_utils;
mod store;

use dotenv::dotenv;
use kafka_utils::kafka_handler::KafkaHandler;
use shaku::{module, HasComponent};
use std::thread;
use utils::add;

use crate::{
    config::config::CONFIG, kafka_utils::kafka_handler::KafkaHandlerImpl, store::store::StoreImpl,
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
async fn main() {
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
        let kafka_handler: &dyn KafkaHandler = APP_MODULE.resolve_ref();
        kafka_handler.start_consuming();
    });

    let result = add(1, 2);
    println!("result is {result}");

    subscribe_thread.join().unwrap();
}
