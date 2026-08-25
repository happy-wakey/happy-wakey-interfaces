//! Static Happy Wakey contract types.
//!
//! This crate intentionally exports declarations only. Transition logic,
//! persistence, HTTP, scheduling, and telemetry belong to consumer crates.

use std::{collections::BTreeMap, future::Future};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppPhase {
    Booting,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthPhase {
    SignedOut,
    Authenticating,
    SignedIn,
    Failed,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum OperationLane {
    Calendar,
    Weather,
    Stocks,
    News,
    OnboardingHydration,
    DesktopNotification,
    CloudNotification,
    CloudReminderSync,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LanePhase {
    Idle,
    Running,
    Ready,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingStep {
    Welcome,
    Account,
    Backup,
    Essentials,
    Ready,
    Complete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LaneSnapshot {
    pub phase: LanePhase,
    pub generation: u64,
    pub active_token: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AppSnapshot {
    pub app_phase: AppPhase,
    pub auth_phase: AuthPhase,
    pub auth_generation: u64,
    pub auth_token: Option<u64>,
    pub onboarding: OnboardingStep,
    pub onboarding_completed_once: bool,
    pub generation: u64,
    pub lanes: BTreeMap<OperationLane, LaneSnapshot>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlarmOccurrenceState {
    Scheduled,
    Firing,
    Acknowledged,
    Snoozed,
    Completed,
    Missed,
    Canceled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlarmTransitionEvent {
    Fire,
    Acknowledge,
    Snooze,
    Complete,
    MarkMissed,
    Cancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransitionDisposition {
    Applied,
    Stale,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Alarm {
    pub id: String,
    pub label: String,
    pub local_time: String,
    pub time_zone: String,
    pub weekdays: Vec<u8>,
    pub enabled: bool,
    pub sound: String,
    pub volume: f32,
    pub gradual_seconds: u32,
    pub tags: Vec<String>,
    pub generation: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AlarmOccurrence {
    pub id: String,
    pub alarm_id: String,
    pub scheduled_for: String,
    pub state: AlarmOccurrenceState,
    pub generation: u64,
    pub active_transition_id: Option<String>,
    pub snooze_until: Option<String>,
    pub acknowledged_at: Option<String>,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAlarmRequest {
    pub transition_id: String,
    pub label: String,
    pub local_time: String,
    pub time_zone: String,
    pub weekdays: Vec<u8>,
    pub enabled: bool,
    pub sound: String,
    pub volume: f32,
    pub gradual_seconds: u32,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransitionAlarmRequest {
    pub transition_id: String,
    pub expected_generation: u64,
    pub event: AlarmTransitionEvent,
    pub snooze_until: Option<String>,
    pub client_time: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransitionAlarmResponse {
    pub disposition: TransitionDisposition,
    pub occurrence: AlarmOccurrence,
    pub error: Option<ApiError>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOperation {
    Upsert,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncChange {
    pub change_id: String,
    pub scope: String,
    pub collection: String,
    pub entity_id: String,
    pub operation: ChangeOperation,
    pub generation: u64,
    pub actor_id: String,
    pub document: Option<Value>,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncEnvelope {
    pub schema: String,
    pub cursor: String,
    pub changes: Vec<SyncChange>,
    pub has_more: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub trace_id: Option<String>,
}

/// Transport-neutral operations supported by the web-to-API service lanes.
///
/// Direct database access implements only this read operation. HTTPS, bounded
/// persistent TLS, and JetStream carry the same request and response types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ServiceOperation {
    ListAlarms,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ServiceOperationStatus {
    Ok,
    Unauthorized,
    Unavailable,
    Invalid,
}

/// A single independently authenticated operation.
///
/// `operation_id` is an RFC 4122 UUID used for JetStream message de-duplication
/// and durable response correlation. The bearer is sensitive runtime data and
/// must never enter telemetry, persistence, or a dead-letter payload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ServiceOperationRequest {
    pub schema: String,
    pub operation_id: String,
    pub bearer_token: String,
    pub operation: ServiceOperation,
}

/// Canonical result shared by the persistent TLS and JetStream lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ServiceOperationResponse {
    pub schema: String,
    pub operation_id: String,
    pub status: ServiceOperationStatus,
    pub alarms: Vec<Alarm>,
    pub error: Option<ApiError>,
}

/// Persistence boundary implemented by the API service, never by this crate.
pub trait AlarmRepository: Send + Sync {
    type Error: Send + Sync + 'static;

    fn list_alarms(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<Vec<Alarm>, Self::Error>> + Send;

    fn create_alarm(
        &self,
        owner_id: &str,
        request: CreateAlarmRequest,
    ) -> impl Future<Output = Result<Alarm, Self::Error>> + Send;

    fn transition_occurrence(
        &self,
        owner_id: &str,
        occurrence_id: &str,
        request: TransitionAlarmRequest,
    ) -> impl Future<Output = Result<TransitionAlarmResponse, Self::Error>> + Send;
}

/// Transport-neutral sync boundary implemented by Happy Wakey Sync.
pub trait SyncTransport: Send + Sync {
    type Error: Send + Sync + 'static;

    fn push(
        &self,
        owner_id: &str,
        changes: Vec<SyncChange>,
    ) -> impl Future<Output = Result<SyncEnvelope, Self::Error>> + Send;

    fn pull(
        &self,
        owner_id: &str,
        cursor: &str,
        limit: u32,
    ) -> impl Future<Output = Result<SyncEnvelope, Self::Error>> + Send;
}
