extern crate serde;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod message;
pub mod protos;

#[cfg(test)]
mod tests {
    use protobuf::{Message, SpecialFields};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn json_gen_test() {
        let message = message::Message {
            id: String::from("id-1"),
            source: String::from("source"),
            time: Some(String::from("time")),
            message_type: message::MessageType::ExampleMessage,
            data_base64: Some(String::from("data_base64")),
            datacontenttype: Some(String::from("datacontenttype")),
            dataschema: Some(String::from("dataschema")),
            subject: Some(String::from("subject")),
            specversion: String::from("specversion"),
            data: message::Data {
                greeting: String::from(format!("Hello World!")),
                style: message::Style::Full,
                thing1: message::Thing1 {
                    thing1_type: message::Thing1Type::Bar,
                    bar: Some(String::from("bar")),
                    foo: None,
                },
                thing2: message::Thing2 {
                    foo: String::from("foo"),
                    bar: String::from("bar"),
                },
                thing3: message::Thing3::Bool(false),
            },
        };

        let json_value = serde_json::to_string(&message).unwrap();

        let message_deserialized: message::Message =
            serde_json::from_str(&json_value.to_string()).unwrap();

        println!("Received message: {}", message.id);

        match &message_deserialized.data.thing3 {
            message::Thing3::Bool(value) => {
                assert_eq!(value, &false);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn proto_gen() {
        let purchase = protos::customer_event::CustomerCloudEvent {
            special_fields: SpecialFields::new(),
            payload: Some(
                protos::customer_event::customer_cloud_event::Payload::Purchase(
                    protos::purchase::PurchaseCloudEvent {
                        id: String::from("id"),
                        source: String::from("source"),
                        spec_version: String::from("0.1.0"),
                        special_fields: SpecialFields::new(),
                        type_:
                            protos::purchase::purchase_cloud_event::Type::EXAMPLE_CUSTOMER_PURCHASE
                                .into(),
                        time: protobuf::MessageField::some(
                            protobuf::well_known_types::timestamp::Timestamp::new(),
                        ),
                        data: Some(protos::purchase::purchase_cloud_event::Data {
                            amount: 12.0,
                            customer_id: String::from("customer1"),
                            item: Some(String::from("item1")),
                            special_fields: SpecialFields::new(),
                        })
                        .into(),
                    },
                ),
            ),
        };
        let serialized = protobuf::Message::write_to_bytes(&purchase);
        assert!(serialized.is_ok());

        let deserialized_customer_event =
            protos::customer_event::CustomerCloudEvent::parse_from_bytes(&serialized.unwrap())
                .unwrap();

        match deserialized_customer_event.payload {
            Some(event) => match event {
                protos::customer_event::customer_cloud_event::Payload::Purchase(purchase) => {
                    assert_eq!(purchase.id, "id");
                    assert_eq!(purchase.data.amount, 12.0);
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
