use crate::config::config;
use chrono;
use kafka::producer::{Producer, Record};
use protobuf::SpecialFields;
use std::{thread, time::Duration};
use utils::kafka::topics::MESSAGES_TOPIC;
use utils::{convert_chrono_to_timestamp, purchase};
use utils::{customer_event, page_view};

pub fn start_producing_purchase_events() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting purchase events producer...");

    for i in 0..i32::MAX {
        thread::sleep(Duration::from_secs(3)); // Simulating work
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
                amount: 1.0 + i as f64,
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
    }
}

pub fn start_producing_page_view_events() {
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let mut producer = Producer::from_hosts(brokers).create().unwrap();
    println!("Starting page view producer...");

    for i in 0..i32::MAX {
        thread::sleep(Duration::from_secs(5)); // Simulating work
        let page_view_event = page_view::PageViewCloudEvent {
            id: String::from(format!("id-{i}")),
            source: String::from("source"),
            spec_version: String::from("0.1.0"),
            special_fields: SpecialFields::new(),
            type_: page_view::page_view_cloud_event::Type::EXAMPLE_CUSTOMER_PAGE_VIEW.into(),
            time: protobuf::MessageField::some(
                convert_chrono_to_timestamp::convert_chrono_to_timestamp(&chrono::Utc::now()),
            ),
            data: protobuf::MessageField::some(page_view::page_view_cloud_event::Data {
                customer_id: String::from("customer1"),
                url: String::from("http://example.com"),
                is_special: false,
                special_fields: SpecialFields::new(),
            }),
        };
        let customer_cloud_event = customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(customer_event::customer_cloud_event::Payload::PageView(
                page_view_event.clone(),
            )),
        };
        let value = protobuf::Message::write_to_bytes(&customer_cloud_event).unwrap();
        producer
            .send(&Record::from_value(MESSAGES_TOPIC, value))
            .unwrap();
        println!("Produced message: {}", page_view_event.id);
    }
}
