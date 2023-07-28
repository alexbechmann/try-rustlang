use super::{balance::Balance, page_view::PageView};
use async_trait::async_trait;
use chrono::Utc;
use mockall::automock;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use shaku::{Component, Interface};
use std::{env, ops::DerefMut};

use utils::{page_view::PageViewCloudEvent, purchase::PurchaseCloudEvent};

#[automock]
#[async_trait]
pub trait Store: Interface {
    async fn update_balance(
        &self,
        purchase_event: &PurchaseCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn increment_page_view(
        &self,
        page_view_event: PageViewCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_balance(&self, customer_id: &str) -> Result<i64, Box<dyn std::error::Error>>;

    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    // fn as_mut(&mut self) -> &mut T;
}

#[derive(Component)]
#[shaku(interface = Store)]
pub struct StoreImpl {
    database: mongodb::Database,
    balances_collection: mongodb::Collection<Balance>,
    page_views_collection: mongodb::Collection<PageView>,
}

#[async_trait]
impl Store for StoreImpl {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client_uri =
            env::var("MONGO_URI").expect("You must set the MONGODB_URI environment var!");
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        let client = Client::with_options(options)?;
        self.database = client.database("balances");
        self.balances_collection = self.database.collection("balances");
        self.page_views_collection = self.database.collection("page_views");
        return Ok(());
    }

    async fn update_balance(
        &self,
        purchase_event: &PurchaseCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        let result = self.balances_collection
            .update_one(
                doc! { "customer_id": purchase_event.data.customer_id.to_string() },
                doc! { "$inc": { "amount":  purchase_event.data.amount as i64 }, "$set": { "updated_at": Utc::now() }},
                options,
            )
            .await;

        match result {
            Ok(result) => {
                println!("Updated balance {} documents", result.modified_count);
            }
            Err(e) => {
                println!("Error updating document: {}", e);
            }
        }
        return Ok(true);
    }

    async fn increment_page_view(
        &self,
        page_view_event: PageViewCloudEvent,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        let result = self
            .page_views_collection
            .update_one(
                doc! { "customer_id": page_view_event.data.customer_id.to_string() },
                doc! { "$inc": { "view_count":  1 }},
                options,
            )
            .await;

        match result {
            Ok(result) => {
                println!("Updated page view {} documents", result.modified_count);
            }
            Err(e) => {
                println!("Error updating document: {}", e);
            }
        }
        return Ok(true);
    }

    async fn get_balance(&self, customer_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let existing_balance = self
            .balances_collection
            .find_one(doc! { "customer_id": customer_id }, None)
            .await;

        if let Some(balance) = existing_balance.unwrap() {
            return Ok(balance.amount);
        } else {
            return Ok(0);
        }
    }
}
