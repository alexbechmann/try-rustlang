use protobuf::well_known_types::timestamp::Timestamp;
use protobuf::SpecialFields;

pub fn convert_chrono_to_timestamp(chrono: &chrono::DateTime<chrono::Utc>) -> Timestamp {
    let seconds = chrono.timestamp();
    let nanos = chrono.timestamp_subsec_nanos();
    Timestamp {
        seconds,
        nanos: nanos as i32,
        special_fields: SpecialFields::new(),
    }
}
