extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod access_control;
pub mod event;
pub mod message;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

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

        let event: event::Event = serde_json::from_str(&json).unwrap();

        match event {
            event::Event::UserPaymentPlanChanged(e) => {
                assert_eq!(e.id, "123");
                assert!(matches!(
                    e.plan,
                    event::EventUserPaymentPlanChangedPlan::Free
                ));
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn access_control_serialize_deserialize() {
        let event = access_control::AccessControl::ExampleAccessControlAccessAssign(
            access_control::AccessControlExampleAccessControlAccessAssign {
                id: "34264c61-9ff4-47ee-802f-9f965f656072".to_string(),
                dataBase64: "_".to_string(),
                datacontenttype: "application/json".to_string(),
                dataschema: "schema1".to_string(),
                source: "rust".to_string(),
                specversion: "1.0.0".to_string(),
                subject: "example".to_string(),
                time: Utc::now().fixed_offset(),
                data: access_control::AccessControlExampleAccessControlAccessAssignData {
                    expiresAt: Some(Box::new(Utc::now().fixed_offset())),
                    id: "1e01c523-0fcf-4923-8999-3ff69f79beaa".to_string(),
                    role: access_control::AccessControlExampleAccessControlAccessAssignDataRole::Reader,
                    userId: "2041b73b-2178-44fc-b74d-c06ec61062f7".to_string()
                },
            },
        );

        let json = serde_json::to_string(&event).unwrap();

        let parsed: access_control::AccessControl = serde_json::from_str(&json).unwrap();

        match parsed {
            access_control::AccessControl::ExampleAccessControlAccessAssign(e) => {
                assert!(matches!(
                    e.data.role,
                    access_control::AccessControlExampleAccessControlAccessAssignDataRole::Reader
                ));
                assert_eq!(e.data.id, "1e01c523-0fcf-4923-8999-3ff69f79beaa");
            }
            _ => panic!("Wrong event type"),
        }
    }
}
