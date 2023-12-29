use syrette::injectable;
use syrette::ptr::TransientPtr;
use syrette::DIContainer;

use crate::{
    kafka_utils::kafka_handler::{KafkaHandler, KafkaHandlerImpl},
    store::store::{Store, StoreImpl},
};

pub fn create_default_container() -> DIContainer {
    let mut container = DIContainer::new();
    container.bind::<dyn Store>().to::<StoreImpl>();
    container
        .bind::<dyn KafkaHandler>()
        .to::<KafkaHandlerImpl>();
    return container;
}
