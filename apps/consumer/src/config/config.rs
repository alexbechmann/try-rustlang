use std::env;

pub struct Config {
    pub kafka_brokers: String,
    pub surreal_config: SurrealConfig,
}

pub struct SurrealConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

pub fn get_config() -> Config {
    Config {
        kafka_brokers: env::var("KAFKA_BROKERS").unwrap(),
        surreal_config: SurrealConfig {
            host: env::var("SURREALDB_HOST").unwrap(),
            port: env::var("SURREALDB_PORT").unwrap(),
            username: env::var("SURREALDB_USER").unwrap(),
            password: env::var("SURREALDB_PASSWORD").unwrap(),
        },
    }
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}
