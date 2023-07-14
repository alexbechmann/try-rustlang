use crate::{config::constants::get_constants, message};
use kafka::consumer::{Consumer, FetchOffset};
use std::str;

pub fn subscribe() {
    println!("subscribe");
    let constants = get_constants();
    let brokers = vec![constants.kafka_brokers];
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

    match &message.data.thing1 {
        message::Thing1Union::Bool(value) => {
            println!("Bool value: {}", value);
        }
        message::Thing1Union::Double(value) => {
            println!("Double value: {}", value);
        }
        message::Thing1Union::String(value) => {
            println!("String value: {}", value);
        }
        message::Thing1Union::Thing1Class(thing1_class) => {
            println!("Thing1Class foo value: {}", thing1_class.foo);
        }
    }
}
