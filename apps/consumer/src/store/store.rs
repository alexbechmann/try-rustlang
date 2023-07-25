use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;
use utils::purchase::PurchaseCloudEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    amount: i64,
    customer_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: chrono::DateTime<Utc>,
}

#[async_trait]
pub trait Store {
    async fn update_balance(
        &self,
        purchase_event: PurchaseCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_balance(customer_id: &str) -> Result<i64, Box<dyn std::error::Error>>;

    fn test_connection() -> bool;
}

pub fn get_store() -> impl Store {
    return StoreImpl::new();
}

pub struct StoreImpl {}

impl StoreImpl {
    pub fn new() -> StoreImpl {
        StoreImpl {}
    }
}

#[async_trait]
impl Store for StoreImpl {
    async fn update_balance(
        &self,
        purchase_event: PurchaseCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let balances_collection = get_balances_collection().await.unwrap();
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        let result = balances_collection
            .update_one(
                doc! { "customer_id": purchase_event.data.customer_id.to_string() },
                doc! { "$inc": { "amount":  purchase_event.data.amount as i64 }, "$set": { "updated_at": Utc::now() }},
                options,
            )
            .await;

        match result {
            Ok(result) => {
                println!("Updated {} documents", result.modified_count);
            }
            Err(e) => {
                println!("Error updating document: {}", e);
            }
        }
        return Ok(true);
    }

    async fn get_balance(customer_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let balances_collection = get_balances_collection().await.unwrap();
        let existing_balance = balances_collection
            .find_one(doc! { "customer_id": customer_id.to_string() }, None)
            .await;

        if let Some(balance) = existing_balance.unwrap() {
            return Ok(balance.amount);
        } else {
            return Ok(0);
        }
    }

    fn test_connection() -> bool {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = runtime.block_on(get_client());
        match client {
            Ok(client) => {
                println!("Connected successfully.");
                let database_names = runtime
                    .block_on(client.list_database_names(None, None))
                    .unwrap();
                for name in database_names {
                    println!("- {}", name);
                }
                return true;
            }
            Err(e) => {
                println!("Error connecting to MongoDB: {}", e);
                return false;
            }
        }
    }
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    let client_uri = env::var("MONGO_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    return Ok(client);
}

async fn get_balances_collection() -> Result<mongodb::Collection<Balance>, mongodb::error::Error> {
    let client = get_client().await.unwrap();
    let database = client.database("balances");
    let balances_collection: mongodb::Collection<Balance> = database.collection("balances");
    return Ok(balances_collection);
}
