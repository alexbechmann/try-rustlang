use crate::config::config;
use kafka::producer::{Producer, Record};
use std::{thread, time::Duration};
use utils::message;

pub fn produce() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let topic = "messages";
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting producer...");

    for i in 0..10 {
        let message = message::Message {
            id: String::from(format_f!("id-{i}")),
            source: String::from("source"),
            time: Some(String::from("time")),
            message_type: message::MessageType::ExampleMessage,
            data_base64: Some(String::from("data_base64")),
            datacontenttype: Some(String::from("datacontenttype")),
            dataschema: Some(String::from("dataschema")),
            subject: Some(String::from("subject")),
            specversion: String::from("specversion"),
            data: message::Data {
                greeting: String::from(format_f!("Hello World! {i}")),
                style: message::Style::Full,
                thing1: message::Thing1 {
                    thing1_type: message::Thing1Type::Bar,
                    bar: Some(String::from("bar")),
                    foo: None,
                },
                thing2: message::Thing2 {
                    foo: String::from("foo"),
                    bar: String::from("bar"),
                },
                thing3: message::Thing3::Bool(false),
            },
        };

        let message_json = serde_json::to_string(&message).unwrap();

        producer
            .send(&Record::from_value(topic, message_json.as_bytes()))
            .unwrap();
        println_f!("Produced message: {message.id}");
        thread::sleep(Duration::from_secs(3)); // Simulating work
    }
}
