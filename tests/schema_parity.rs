use happy_wakey_interfaces::{
    Alarm, AlarmOccurrence, AppSnapshot, CreateAlarmRequest, ServiceOperationRequest,
    ServiceOperationResponse, SyncEnvelope, TransitionAlarmRequest, TransitionAlarmResponse,
};
use schemars::schema_for;

#[test]
fn every_public_wire_type_has_a_generated_schema() {
    for schema in [
        schema_for!(Alarm),
        schema_for!(AlarmOccurrence),
        schema_for!(AppSnapshot),
        schema_for!(CreateAlarmRequest),
        schema_for!(ServiceOperationRequest),
        schema_for!(ServiceOperationResponse),
        schema_for!(SyncEnvelope),
        schema_for!(TransitionAlarmRequest),
        schema_for!(TransitionAlarmResponse),
    ] {
        let encoded = serde_json::to_value(schema).expect("schema serializes");
        assert_eq!(
            encoded.get("$schema").and_then(|v| v.as_str()),
            Some("https://json-schema.org/draft/2020-12/schema")
        );
        assert!(encoded.get("type").is_some() || encoded.get("$ref").is_some());
    }
}

#[test]
fn repository_contains_only_declarations() {
    let source = include_str!("../src/lib.rs");
    assert!(!source.contains("impl AlarmRepository for"));
    assert!(!source.contains("impl SyncTransport for"));
    for forbidden in ["reqwest", "sea_orm", "sqlx", "axum", "tokio::net"] {
        assert!(
            !source.contains(forbidden),
            "implementation dependency leaked: {forbidden}"
        );
    }
}
