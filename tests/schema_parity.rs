use happy_wakey_interfaces::{
    Alarm, AlarmOccurrence, AppSnapshot, AsyncOperationAccepted, AsyncOperationRequest,
    AsyncOperationSignal, ChatSession, ConnectorConsent, CorrelationFinding, CreateAlarmRequest,
    EmbeddingDescriptor, MorningBriefing, OnboardingIntent, RealtimeEnvelope,
    ServiceOperationRequest, ServiceOperationResponse, SourceItemCandidate, SyncEnvelope,
    TransitionAlarmRequest, TransitionAlarmResponse, UsefulnessDecision,
};
use schemars::schema_for;

#[test]
fn every_public_wire_type_has_a_generated_schema() {
    for schema in [
        schema_for!(Alarm),
        schema_for!(AlarmOccurrence),
        schema_for!(AppSnapshot),
        schema_for!(AsyncOperationAccepted),
        schema_for!(AsyncOperationRequest),
        schema_for!(AsyncOperationSignal),
        schema_for!(CreateAlarmRequest),
        schema_for!(ServiceOperationRequest),
        schema_for!(ServiceOperationResponse),
        schema_for!(SyncEnvelope),
        schema_for!(TransitionAlarmRequest),
        schema_for!(TransitionAlarmResponse),
        schema_for!(OnboardingIntent),
        schema_for!(ConnectorConsent),
        schema_for!(SourceItemCandidate),
        schema_for!(UsefulnessDecision),
        schema_for!(MorningBriefing),
        schema_for!(EmbeddingDescriptor),
        schema_for!(CorrelationFinding),
        schema_for!(RealtimeEnvelope),
        schema_for!(ChatSession),
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
    assert!(!source.contains("impl BriefingRepository for"));
    for forbidden in ["reqwest", "sea_orm", "sqlx", "axum", "tokio::net"] {
        assert!(
            !source.contains(forbidden),
            "implementation dependency leaked: {forbidden}"
        );
    }
}
