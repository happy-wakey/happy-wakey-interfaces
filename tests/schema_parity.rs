use happy_wakey_interfaces::{
    Alarm, AlarmOccurrence, AppSnapshot, AsyncOperationAccepted, AsyncOperationRequest,
    AsyncOperationSignal, BiometricSummary, Briefing, ConnectProviderRequest, CreateAlarmRequest,
    CreateHabitRequest, DayPlan, EnvironmentBrief, Habit, HabitEntry, InboxDigest, MarketWatch,
    MessageDigest, ModuleLayout, OperationLane, ProviderConnection, RecordHabitEntryRequest,
    ServiceOperationRequest, ServiceOperationResponse, SleepSummary, SyncEnvelope,
    TransitionAlarmRequest, TransitionAlarmResponse,
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
        schema_for!(BiometricSummary),
        schema_for!(Briefing),
        schema_for!(ConnectProviderRequest),
        schema_for!(CreateAlarmRequest),
        schema_for!(CreateHabitRequest),
        schema_for!(DayPlan),
        schema_for!(EnvironmentBrief),
        schema_for!(Habit),
        schema_for!(HabitEntry),
        schema_for!(InboxDigest),
        schema_for!(MarketWatch),
        schema_for!(MessageDigest),
        schema_for!(ModuleLayout),
        schema_for!(ProviderConnection),
        schema_for!(RecordHabitEntryRequest),
        schema_for!(ServiceOperationRequest),
        schema_for!(ServiceOperationResponse),
        schema_for!(SleepSummary),
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

/// The Rust lane enum and the hand-written JSON Schema are two spellings of one
/// list, and nothing else forces them to agree. A lane added to one and not the
/// other produces snapshots a peer rejects, so the two are compared directly.
#[test]
fn operation_lanes_match_the_app_snapshot_schema() {
    const LANES: [OperationLane; 21] = [
        OperationLane::Calendar,
        OperationLane::Weather,
        OperationLane::Stocks,
        OperationLane::News,
        OperationLane::OnboardingHydration,
        OperationLane::DesktopNotification,
        OperationLane::CloudNotification,
        OperationLane::CloudReminderSync,
        OperationLane::Bluetooth,
        OperationLane::Briefing,
        OperationLane::DayPlan,
        OperationLane::Tasks,
        OperationLane::Habits,
        OperationLane::Focus,
        OperationLane::Sleep,
        OperationLane::Biometrics,
        OperationLane::Inbox,
        OperationLane::Messages,
        OperationLane::Commute,
        OperationLane::Portfolio,
        OperationLane::ProviderSync,
    ];

    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/app-snapshot.schema.json"))
            .expect("app snapshot schema parses");
    let declared: Vec<String> = schema["properties"]["lanes"]["propertyNames"]["enum"]
        .as_array()
        .expect("lane property names are enumerated")
        .iter()
        .map(|value| value.as_str().expect("lane name is a string").to_owned())
        .collect();

    let encoded: Vec<String> = LANES
        .iter()
        .map(|lane| {
            serde_json::to_value(lane)
                .expect("lane serializes")
                .as_str()
                .expect("lane serializes to a string")
                .to_owned()
        })
        .collect();

    assert_eq!(
        encoded, declared,
        "OperationLane and app-snapshot.schema.json disagree"
    );
}

/// Every schema file must be reachable from the crate, so a contract cannot be
/// added as JSON alone and quietly skip the Rust side of the boundary.
#[test]
fn every_schema_file_is_valid_json_with_an_id() {
    for source in [
        include_str!("../schemas/provider-connection.schema.json"),
        include_str!("../schemas/sleep-summary.schema.json"),
        include_str!("../schemas/biometric-summary.schema.json"),
        include_str!("../schemas/day-plan.schema.json"),
        include_str!("../schemas/habit.schema.json"),
        include_str!("../schemas/inbox-digest.schema.json"),
        include_str!("../schemas/message-digest.schema.json"),
        include_str!("../schemas/market-watch.schema.json"),
        include_str!("../schemas/environment-brief.schema.json"),
        include_str!("../schemas/briefing.schema.json"),
        include_str!("../schemas/module-layout.schema.json"),
    ] {
        let document: serde_json::Value =
            serde_json::from_str(source).expect("schema file parses as JSON");
        let id = document["$id"].as_str().expect("schema declares an $id");
        assert!(
            id.starts_with("https://schemas.happy-wakey.dev/"),
            "unexpected schema $id: {id}"
        );
        assert_eq!(
            document["$schema"].as_str(),
            Some("https://json-schema.org/draft/2020-12/schema")
        );
    }
}

#[test]
fn repository_contains_only_declarations() {
    let source = include_str!("../src/lib.rs");
    assert!(!source.contains("impl AlarmRepository for"));
    assert!(!source.contains("impl SyncTransport for"));
    assert!(!source.contains("impl DashboardRepository for"));
    assert!(!source.contains("impl DigestComposer for"));
    for forbidden in ["reqwest", "sea_orm", "sqlx", "axum", "tokio::net"] {
        assert!(
            !source.contains(forbidden),
            "implementation dependency leaked: {forbidden}"
        );
    }
}
