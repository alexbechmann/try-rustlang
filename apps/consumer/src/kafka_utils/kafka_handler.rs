use std::sync::Arc;

use crate::store::{
    self,
    store::{Store, StoreImpl},
};

pub trait KafkaHandler {}

// #[inject]
struct KafkaHandlerImpl {
    // store: dyn store::store::Store,
    // store: Arc<dyn Store>,
}

// impl KafkaHandlerImpl {
//     #[inject]
//     pub fn new(store: dyn Store) -> Self {
//         Self { store }
//     }
// }
