extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod message;
mod utils;
use serde_json::json;
use utils::add::add;


fn main() {
    let source = "rust";
    println!("source is {source}");

    let json_value = json!({
        "id": "1", 
        "source": "{source}", 
        "time": "", 
        "specversion": "1", 
        "type": "example.message" ,
        "data": {
            "greeting": "Hello World!",
            "style": "full"
        }
    });

    let message: message::Message = serde_json::from_str(&json_value.to_string()).unwrap();

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
            style: message::Style::Full
        }
    };
    
    let serialized_access = serde_json::to_string(&message_new).unwrap();
    
    println!("{} ... ", message.source);
    println!("{}", serialized_access);

    let result = add(1, 2);
    println!("result: {result}");
}