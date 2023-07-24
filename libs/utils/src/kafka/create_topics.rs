use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    ClientConfig,
};

use crate::kafka::topics::MESSAGES_TOPIC;

pub async fn create_topics(kafka_brokers: &str) {
    println!("Creating topics...");
    let topics = vec![NewTopic::new(MESSAGES_TOPIC, 1, TopicReplication::Fixed(1))];
    let admin_options = AdminOptions::default();
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers.to_string())
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap();

    let result = admin_client.create_topics(&topics, &admin_options).await;

    match result {
        Ok(_) => println!("Topic created"),
        Err(e) => println!("Error creating topic: {:?}", e),
    }
}
