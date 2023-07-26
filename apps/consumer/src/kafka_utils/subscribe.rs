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
    let runtime = runtime::Runtime::new().unwrap();
    let store = get_store();
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
                let _ = runtime.block_on(handle_event(m.value, &store));
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}

async fn handle_event(bytes: &[u8], store: &impl Store) {
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
            let balance = store
                .get_balance(data.customer_id.to_string())
                .await
                .unwrap();
            println!(
                "Balance for {} is {}",
                data.customer_id.to_string(),
                balance
            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::store::MockStore;
    use chrono::Utc;
    use protobuf::SpecialFields;
    use utils::{convert_chrono_to_timestamp::convert_chrono_to_timestamp, page_view, purchase};

    #[tokio::test]
    async fn handle_purchase_event_test() {
        let mut store = MockStore::new();
        store
            .expect_update_balance()
            .times(1)
            .returning(|purchase_event| {
                println!(
                    "Mocked update_balance for {}",
                    purchase_event.data.customer_id
                );
                return Ok(true);
            });
        let customer_event = customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(customer_event::customer_cloud_event::Payload::Purchase(
                purchase::PurchaseCloudEvent {
                    id: String::from("id"),
                    source: String::from("source"),
                    spec_version: String::from("0.1.0"),
                    special_fields: SpecialFields::new(),
                    type_: purchase::purchase_cloud_event::Type::EXAMPLE_CUSTOMER_PURCHASE.into(),
                    time: protobuf::MessageField::some(convert_chrono_to_timestamp(&Utc::now())),
                    data: protobuf::MessageField::some(purchase::purchase_cloud_event::Data {
                        amount: 12.0,
                        customer_id: String::from("customer1"),
                        item: Some(String::from("item1")),
                        special_fields: SpecialFields::new(),
                    }),
                },
            )),
        };
        let serialized = protobuf::Message::write_to_bytes(&customer_event).unwrap();
        let _result = handle_event(&serialized, &store).await;
        assert!(true);
    }

    #[tokio::test]
    async fn handle_page_view_event_test() {
        let mut store = MockStore::new();
        store
            .expect_increment_page_view()
            .times(1)
            .returning(|page_view_event| {
                println!(
                    "Mocked increment_page_view for {}",
                    page_view_event.data.customer_id
                );
                return Ok(true);
            });
        let customer_event = customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(customer_event::customer_cloud_event::Payload::PageView(
                page_view::PageViewCloudEvent {
                    id: String::from("id"),
                    source: String::from("source"),
                    spec_version: String::from("0.1.0"),
                    special_fields: SpecialFields::new(),
                    type_: page_view::page_view_cloud_event::Type::EXAMPLE_CUSTOMER_PAGE_VIEW
                        .into(),
                    time: protobuf::MessageField::some(convert_chrono_to_timestamp(&Utc::now())),
                    data: protobuf::MessageField::some(page_view::page_view_cloud_event::Data {
                        customer_id: String::from("customer1"),
                        url: String::from("http://example.com"),
                        is_special: false,
                        special_fields: SpecialFields::new(),
                    }),
                },
            )),
        };
        let serialized = protobuf::Message::write_to_bytes(&customer_event).unwrap();
        let _result = handle_event(&serialized, &store).await;
        assert!(true);
    }
}
