use std::env;

pub struct Config {
    pub kafka_brokers: String,
}

pub fn get_config() -> Config {
    Config {
        kafka_brokers: env::var("KAFKA_BROKERS").unwrap(),
    }
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}
