extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod message;
mod utils;
mod kafka_handler;

use serde_json::json;
use utils::add::add;


fn main() {
    let source = "rust";
    println!("source is {source}");

    kafka_handler::subscribe::subscribe();

    let json_value = json!({
        "id": "1", 
        "source": "{source}", 
        "time": "", 
        "specversion": "1", 
        "type": "example.message" ,
        "data": {
            "greeting": "Hello World!",
            "style": "full",
            "thing1": false,
            "thing2": {
                "foo": "foo",
                "bar": "bar"
            }
        }
    });

    let message: message::Message = serde_json::from_str(&json_value.to_string()).unwrap();
    
    match &message.data.thing1 {
        message::Thing1Union::Bool(value) => {
            // Code to handle Bool variant
            println!("Bool value: {}", value);
        }
        message::Thing1Union::Double(value) => {
            // Code to handle Double variant
            println!("Double value: {}", value);
        }
        message::Thing1Union::String(value) => {
            // Code to handle String variant
            println!("String value: {}", value);
        }
        message::Thing1Union::Thing1Class(thing1_class) => {
            // Code to handle Thing1Class variant
            println!("Thing1Class foo value: {}", thing1_class.foo);
        }
    }

    let message_new = message::Message{
        id: String::from("id"),
        source: String::from("source"),
        time: Some(String::from("time")),
        message_type: message::Type::ExampleMessage,
        data_base64: Some(String::from("data_base64")),
        datacontenttype: Some(String::from("datacontenttype")),
        dataschema: Some(String::from("dataschema")),
         subject: Some(String::from("subject")),
        specversion: String::from("specversion"),
        data: message::Data {
            greeting: String::from("Hello World!"),
            style: message::Style::Full,
            thing1: message::Thing1Union::Thing1Class(message::Thing1Class {
                foo: String::from("id"),
            }),
            thing2: message::Thing2{foo: String::from("foo"), bar: String::from("bar")}
        }
    };

    
    let serialized_access = serde_json::to_string(&message_new).unwrap();
    
    println!("{} ... ", message.source);
    println!("{}", serialized_access);

    let result = add(1, 2);
    println!("result: {result}");
}