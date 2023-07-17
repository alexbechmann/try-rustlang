use crate::config::config;
use kafka::consumer::{Consumer, FetchOffset};
use std::str;
use utils::message;

pub fn subscribe() {
    println!("subscribe");
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let topic = "messages";
    let group_id = "my-group".to_string();
    let mut consumer = Consumer::from_hosts(brokers)
        .with_topic(topic.to_owned())
        .with_group(group_id)
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let json_value = str::from_utf8(m.value).unwrap();
                handle_event(json_value);
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}

fn handle_event(json_value: &str) {
    let message: message::Message = serde_json::from_str(&json_value.to_string()).unwrap();

    println!("Received message: {}", message.id);

    match &message.data.thing3 {
        message::Thing3::Bool(value) => {
            println_f!("Bool value: {value}");
        }
        message::Thing3::Double(value) => {
            println_f!("Double value: {value}");
        }
        message::Thing3::String(value) => {
            println_f!("String value: {value}");
        }
    }
}
