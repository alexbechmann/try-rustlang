use crate::config::config;
use chrono;
use kafka::producer::{Producer, Record};
use protobuf::SpecialFields;
use std::{thread, time::Duration};
use utils::customer_event;
use utils::kafka::topics::MESSAGES_TOPIC;
use utils::purchase;

pub fn produce() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting producer...");

    for i in 0..i32::MAX {
        let purchase_event = purchase::PurchaseCloudEvent {
            id: String::from(format!("id-{i}")),
            source: String::from("try-rustlang-producer"),
            spec_version: String::from("0.1.0"),
            special_fields: SpecialFields::new(),
            type_: purchase::purchase_cloud_event::Type::EXAMPLE_CUSTOMER_PURCHASE.into(),
            time: protobuf::MessageField::some(
                utils::convert_chrono_to_timestamp::convert_chrono_to_timestamp(&chrono::Utc::now()),
            ),
            data: protobuf::MessageField::some(purchase::purchase_cloud_event::Data {
                amount: 12.0,
                customer_id: String::from(format!("customer1")),
                item: Some(String::from("item1")),
                special_fields: SpecialFields::new(),
            }),
        };
        let customer_cloud_event = customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(customer_event::customer_cloud_event::Payload::Purchase(
                purchase_event.clone(),
            )),
        };
        let value = protobuf::Message::write_to_bytes(&customer_cloud_event).unwrap();
        producer
            .send(&Record::from_value(MESSAGES_TOPIC, value))
            .unwrap();
        println!("Produced message: {}", purchase_event.id);
        thread::sleep(Duration::from_secs(3)); // Simulating work
    }
}
