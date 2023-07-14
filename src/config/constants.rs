
use std::env;

pub struct Constants {
    pub kafka_brokers: String,
}

pub fn get_constants() -> Constants {
    Constants {
        kafka_brokers: env::var("KAFKA_BROKERS").unwrap(),
    }
}
