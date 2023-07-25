use crate::{
    config::config,
    store::store::{get_store, Store},
};
use kafka::consumer::{Consumer, FetchOffset};
use protobuf::Message;
use tokio::runtime;
use utils::{customer_event, kafka::topics::MESSAGES_TOPIC};

pub fn subscribe() {
    println!("subscribe");
    let brokers = vec![config::CONFIG.kafka_brokers.to_string()];
    let group_id = "my-group".to_string();
    let mut consumer = Consumer::from_hosts(brokers)
        .with_topic(MESSAGES_TOPIC.to_string())
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
    let customer_cloud_event =
        customer_event::CustomerCloudEvent::parse_from_bytes(&bytes).unwrap();

    let runtime = runtime::Runtime::new().unwrap();

    match customer_cloud_event.payload.unwrap() {
        customer_event::customer_cloud_event::Payload::Purchase(purchase_event) => {
            println!(
                "Received purchase event: {} from {}",
                purchase_event.id, purchase_event.source
            );
            let store = &get_store();
            let future = store.update_balance(purchase_event);
            let _ = runtime.block_on(future);
        }
        customer_event::customer_cloud_event::Payload::PageView(page_view_event) => {
            println!(
                "Received event: {} from {}",
                page_view_event.id, page_view_event.source
            )
        }
        _ => panic!("Unhandled type"),
    }
}
