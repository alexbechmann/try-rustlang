use std::sync::Arc;

use async_trait::async_trait;
use kafka::consumer::{Consumer, FetchOffset};
use mockall::automock;
use protobuf::Message;
use shaku::{Component, Interface};
use tokio::runtime;
use utils::{customer_event, kafka::topics::MESSAGES_TOPIC};

use crate::{
    config,
    store::store::{Store, StoreImpl},
};

#[automock]
#[async_trait]
pub trait KafkaHandler: Interface {
    fn start_consuming(&self);
}

#[derive(Component)]
#[shaku(interface = KafkaHandler)]
pub struct KafkaHandlerImpl {
    #[shaku(inject)]
    store: Arc<dyn Store>,
}

#[async_trait]
impl KafkaHandler for KafkaHandlerImpl {
    fn start_consuming(&self) {
        let store = &self.store;
        println!("start_consuming {:#?}", store.type_id());
        println!("subscribe");
        let runtime = runtime::Runtime::new().unwrap();
        // let store = get_store();
        let brokers = vec![config::config::CONFIG.kafka_brokers.to_string()];
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
                    let _ = runtime.block_on(self.handle_event(m.value));
                }
                consumer.consume_messageset(ms).unwrap();
            }
            consumer.commit_consumed().unwrap();
        }
    }
}

impl KafkaHandlerImpl {
    async fn handle_event(&self, bytes: &[u8]) {
        let store = &self.store;
        let customer_cloud_event =
            customer_event::CustomerCloudEvent::parse_from_bytes(&bytes).unwrap();

        match customer_cloud_event.payload.unwrap() {
            customer_event::customer_cloud_event::Payload::Purchase(purchase_event) => {
                println!(
                    "Received purchase event: {} from {}",
                    purchase_event.id, purchase_event.source
                );
                let _ = store.update_balance(&purchase_event).await;
                let data = Box::new(purchase_event.data);
                // let balance = store
                //     .get_balance(data.customer_id.to_string())
                //     .await
                //     .unwrap();
                // println!(
                //     "Balance for {} is {}",
                //     data.customer_id.to_string(),
                //     balance
                // );
            }
            customer_event::customer_cloud_event::Payload::PageView(page_view_event) => {
                println!(
                    "Received event: {} from {}",
                    page_view_event.id, page_view_event.source
                );
                let _ = store.increment_page_view(page_view_event).await;
            }
            _ => panic!("Unhandled type"),
        }
    }
}
