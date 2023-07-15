extern crate serde;
#[macro_use]
extern crate serde_derive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod event;
pub mod message;

#[cfg(test)]
mod tests {
    use super::*;
    use event;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn codegen_serialize_test() {
        let user_created_event = event::Event::UserCreated(event::EventUserCreated {
            id: "123".to_string(),
        });

        let json = serde_json::to_string(&user_created_event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let event_type = &parsed["eventType"];

        assert_eq!(event_type, "USER_CREATED");
    }

    #[test]
    fn codegen_deserialize_test() {
        let json = r#"{
            "eventType": "USER_PAYMENT_PLAN_CHANGED",
            "id": "123",
            "plan": "FREE"
        }"#;

        let event: event::Event = serde_json::from_str(&json.to_string()).unwrap();

        match (event) {
            event::Event::UserPaymentPlanChanged(event::EventUserPaymentPlanChanged {
                id,
                plan,
            }) => {
                assert_eq!(id, "123");
                match plan {
                    event::EventUserPaymentPlanChangedPlan::Free => assert!(true),
                    _ => assert!(false),
                }
            }
            _ => panic!("Wrong event type"),
        }
    }
}
