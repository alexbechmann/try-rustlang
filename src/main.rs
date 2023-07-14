extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod access_control;
mod utils;
use serde_json::json;
use utils::add::add;


fn main() {
    let source = "rust";
    let _j = r#"{
        "id": "1",
        "source": "{source}",
    }"#;

    println!("source is {source}");


    let json2 = json!({
        "id": "1", 
        "source": "{source}", 
        "time": "", 
        "specversion": "1", 
        "type": "vipe.access-control.access.assign.home" ,
        "data": {}
    });

    let access: access_control::AccessControl = serde_json::from_str(&json2.to_string()).unwrap();

    let access_new = access_control::AccessControl {
        id: String::from("id"),
        source: String::from("source"),
        time: Some(String::from("time")),
        access_control_type: access_control::Type::VipeAccessControlAccessAssignHome,
        data_base64: Some(String::from("data_base64")),
        datacontenttype: Some(String::from("datacontenttype")),
        dataschema: Some(String::from("dataschema")),
         subject: Some(String::from("subject")),
        specversion: String::from("specversion"),
        data: access_control::Data {
            home_uri: Some(String::from("home_uri")),
            assigned_by: Some(String::from("assigned_by")),
            expires_at: Some(String::from("expires_at")),
            new_user_id: Some(String::from("new_user_id")),
            role: Some(access_control::AccessRole::Owner),
            old_user_id: Some(String::from("old_user_id")),
            room_uri: Some(String::from("room_uri")),
            support_id: Some(String::from("support_id")),
            uri: Some(String::from("uri")),
            user_id: Some(String::from("user_id")),
        }
    };
    
    let serialized_access = serde_json::to_string(&access_new).unwrap();
    
    println!("{} ... ", access.source);
    println!("{}", serialized_access);

    let result = add(1, 2);
    println!("result: {result}");
}