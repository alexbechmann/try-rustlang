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

        let deserialized_customer_event =
            protos::customer_event::CustomerCloudEvent::parse_from_bytes(&serialized.unwrap())
                .unwrap();

        assert_eq!(deserialized_customer_event.id, "id");

        match deserialized_customer_event.data {
            Some(data) => match data {
                protos::customer_event::customer_cloud_event::Data::Purchase(purchase) => {
                    assert_eq!(purchase.amount, 12.0)
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
