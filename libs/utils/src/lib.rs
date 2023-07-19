extern crate serde;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod message;
pub mod protos;

#[cfg(test)]
mod tests {
    use protobuf::SpecialFields;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn proto_gen() {
        let purchase = protos::customer_event::CustomerEvent {
            id: "id".to_string(),
            action: Some(protos::customer_event::customer_event::Action::Purchase(
                protos::purchase::Purchase {
                    amount: 12.0,
                    customer_id: "customer1".to_string(),
                    item: "item1".to_string(),
                    special_fields: SpecialFields::new(),
                },
            )),
            special_fields: SpecialFields::new(),
        };
        let serialized = protobuf::Message::write_to_bytes(&purchase);
        assert!(serialized.is_ok());
    }
}
