use crate::config::config;
use kafka::producer::{Producer, Record};
use protobuf::SpecialFields;
use std::{thread, time::Duration};
use utils::{message, protos};

pub fn produce() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let topic = "messages";
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting producer...");

    for i in 0..i32::MAX {
        let purchase: protos::customer_event::CustomerCloudEvent =
            protos::customer_event::CustomerCloudEvent {
                id: String::from(format_f!("id-{i}")),
                source: String::from("try-rustlang-producer"),
                spec_version: String::from("0.1.0"),
                type_: protos::customer_event::customer_cloud_event::Type::PURCHASE.into(),
                data: Some(
                    protos::customer_event::customer_cloud_event::Data::Purchase(
                        protos::purchase::Purchase {
                            amount: 12.0,
                            customer_id: String::from(format_f!("customer-{i}")),
                            item: Some(String::from("item1")),
                            special_fields: SpecialFields::new(),
                        },
                    ),
                ),
                special_fields: SpecialFields::new(),
            };
        let value = protobuf::Message::write_to_bytes(&purchase).unwrap();
        producer.send(&Record::from_value(topic, value)).unwrap();
        println_f!("Produced message: {purchase.id}");
        thread::sleep(Duration::from_secs(3)); // Simulating work
    }
}
