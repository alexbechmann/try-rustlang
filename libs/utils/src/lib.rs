extern crate serde;
#[macro_use]
extern crate serde_derive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod access_control;
pub mod event;
pub mod message;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
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
        let event = access_control::AccessControl::VipeAccessControlAccessAssignHome(
            access_control::AccessControlVipeAccessControlAccessAssignHome {
                id: "123".to_string(),
                dataBase64: "123".to_string(),
                datacontenttype: "123".to_string(),
                dataschema: "123".to_string(),
                source: "123".to_string(),
                specversion: "123".to_string(),
                subject: "123".to_string(),
                time: Utc::now().fixed_offset(),
                data: access_control::AccessControlVipeAccessControlAccessAssignHomeData { 
                    // assignedBy: "123".to_string(),
                    // expiresAt:  Utc::now().fixed_offset(),
                    // expiresAt:  "".to_string(),
                    assignedBy: None,
                    expiresAt: None,
                    supportId: None,
                    homeUri: "123".to_string(),
                    role: access_control::AccessControlVipeAccessControlAccessAssignHomeDataRole::Installer,
                    // supportId: "123".to_string(),
                    uri: "123".to_string(),
                    userId: "123".to_string(),
                }
            },
        );

        let json = serde_json::to_string(&event).unwrap();

        let parsed: access_control::AccessControl = serde_json::from_str(&json).unwrap();

        match parsed {
            access_control::AccessControl::VipeAccessControlAccessAssignHome(e) => {
                assert_eq!(e.id, "123");
                // assert_eq!(e.data.assignedBy, "123");
                // assert_eq!(e.data.expiresAt, "");
                assert_eq!(e.data.homeUri, "123");
                assert!(matches!(e.data.role, access_control::AccessControlVipeAccessControlAccessAssignHomeDataRole::Installer));
                // assert_eq!(e.data.supportId, "123");
                assert_eq!(e.data.uri, "123");
                assert_eq!(e.data.userId, "123");
            }
            _ => panic!("Wrong event type"),
        }
    }
}
