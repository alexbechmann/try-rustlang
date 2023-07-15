use std::env;

pub struct Config {
    pub kafka_brokers: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        kafka_brokers: env::var("KAFKA_BROKERS").unwrap(),
    };
}
