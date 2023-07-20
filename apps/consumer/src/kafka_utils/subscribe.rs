use crate::config::config;
use kafka::consumer::{Consumer, FetchOffset};
use protobuf::Message;

use utils::{protos};

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
                handle_event(m.value);
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}

fn handle_event(bytes: &[u8]) {
    let event = protos::customer_event::CustomerCloudEvent::parse_from_bytes(&bytes).unwrap();
    println_f!("Received event: {event.id} from {event.source}");

    match event.data {
        Some(data) => match data {
            protos::customer_event::customer_cloud_event::Data::Purchase(purchase) => {
                println_f!("Purchase received with amount {purchase.amount}")
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}
