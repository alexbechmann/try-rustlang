use crate::{config::constants::get_constants, message};
use kafka::producer::{Producer, Record};
use std::{thread, time::Duration};

pub fn produce() {
    let constants = get_constants();
    let brokers = vec![constants.kafka_brokers];
    let topic = "messages";
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting producer...");

    for i in 0..10 {
        let message = message::Message {
            id: String::from(format!("id-{}", i)),
            source: String::from("source"),
            time: Some(String::from("time")),
            message_type: message::Type::ExampleMessage,
            data_base64: Some(String::from("data_base64")),
            datacontenttype: Some(String::from("datacontenttype")),
            dataschema: Some(String::from("dataschema")),
            subject: Some(String::from("subject")),
            specversion: String::from("specversion"),
            data: message::Data {
                greeting: String::from(format!("Hello World! {} ", i)),
                style: message::Style::Full,
                thing1: message::Thing1Union::Thing1Class(message::Thing1Class {
                    foo: String::from("id"),
                }),
                thing2: message::Thing2 {
                    foo: String::from("foo"),
                    bar: String::from("bar"),
                },
            },
        };

        let message_json = serde_json::to_string(&message).unwrap();

        producer
            .send(&Record::from_value(topic, message_json.as_bytes()))
            .unwrap();
        println!("Produced message: {} ", message.id);
        thread::sleep(Duration::from_secs(3)); // Simulating work
    }
}
