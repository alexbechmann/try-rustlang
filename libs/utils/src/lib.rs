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
        let purchase = protos::customer_event::CustomerCloudEvent {
            id: String::from("id"),
            source: String::from("source"),
            spec_version: String::from("0.1.0"),
            type_: protos::customer_event::customer_cloud_event::Type::PURCHASE.into(),
            data: Some(
                protos::customer_event::customer_cloud_event::Data::Purchase(
                    protos::purchase::Purchase {
                        amount: 12.0,
                        customer_id: String::from("customer1"),
                        item: Some(String::from("item1")),
                        special_fields: SpecialFields::new(),
                    },
                ),
            ),
            special_fields: SpecialFields::new(),
        };
        let serialized = protobuf::Message::write_to_bytes(&purchase);
        assert!(serialized.is_ok());
    }
}
