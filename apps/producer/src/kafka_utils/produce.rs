use crate::config::config;
use kafka::producer::{Producer, Record};
use protobuf::SpecialFields;
use std::{thread, time::Duration};
use utils::protos;

pub fn produce() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let topic = "messages";
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting producer...");

    for i in 0..i32::MAX {
        let purchase_event = protos::purchase::PurchaseCloudEvent {
            id: String::from(format_f!("id-{i}")),
            source: String::from("try-rustlang-producer"),
            spec_version: String::from("0.1.0"),
            special_fields: SpecialFields::new(),
            type_: protos::purchase::purchase_cloud_event::Type::EXAMPLE_CUSTOMER_PURCHASE.into(),
            time: protobuf::MessageField::some(
                protobuf::well_known_types::timestamp::Timestamp::new(),
            ),
            data: protobuf::MessageField::some(protos::purchase::purchase_cloud_event::Data {
                amount: 12.0,
                customer_id: String::from(format_f!("customer-{i}")),
                item: Some(String::from("item1")),
                special_fields: SpecialFields::new(),
            }),
        };
        let customer_cloud_event = protos::customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(
                protos::customer_event::customer_cloud_event::Payload::Purchase(
                    purchase_event.clone(),
                ),
            ),
        };
        let value = protobuf::Message::write_to_bytes(&customer_cloud_event).unwrap();
        producer.send(&Record::from_value(topic, value)).unwrap();
        println_f!("Produced message: {purchase_event.id}");
        thread::sleep(Duration::from_secs(3)); // Simulating work
    }
}
