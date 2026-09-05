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
    Bluetooth,
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

/// Body used to register an asynchronous operation over authenticated HTTPS.
/// The bearer remains in the HTTP authorization header and is never copied
/// into this durable record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AsyncOperationRequest {
    pub schema: String,
    pub operation_id: String,
    pub operation: ServiceOperation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AsyncOperationAccepted {
    pub schema: String,
    pub operation_id: String,
    pub response_subject: String,
}

/// Credential-free JetStream wake-up for a previously authenticated outbox
/// row. The API derives the verified owner only from that row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AsyncOperationSignal {
    pub schema: String,
    pub operation_id: String,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AccountKind {
    Individual,
    OrganizationMember,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorKind {
    Email,
    Whatsapp,
    Linkedin,
    XDm,
    Slack,
    Teams,
    Calendar,
    Weather,
    Flights,
    Markets,
    Crm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConsentState {
    Pending,
    Granted,
    Revoked,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SenderClass {
    Vip,
    Known,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UsefulnessDisposition {
    Useful,
    NotUseful,
    NeedsReview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UsefulnessReason {
    VipSender,
    ReplyRequested,
    BlockingOthers,
    TimeSensitive,
    TravelDisruption,
    SecurityRisk,
    FinancialImpact,
    CustomerEscalation,
    LowSignal,
    Promotional,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BriefingCardKind {
    ThisDayInHistory,
    UsefulMessage,
    EmailBottleneck,
    TeamBottleneck,
    Calendar,
    Weather,
    ExtendedOutlook,
    Flight,
    Market,
    Kpi,
    Task,
    News,
    AudioBriefing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BriefingCardPriority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    Ephemeral,
    Standard,
    LegalHold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MultipleTestingCorrection {
    None,
    Bonferroni,
    BenjaminiHochberg,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeTransport {
    Websocket,
    TlsTcp,
    Nats,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChatAudience {
    SalesVisitor,
    CustomerSupport,
    OrganizationAdmin,
    InternalOperator,
    Owner,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OnboardingIntent {
    pub request_id: String,
    pub account_kind: AccountKind,
    pub organization_name: Option<String>,
    pub requested_seat_count: Option<u32>,
    pub time_zone: String,
    pub morning_window_start: String,
    pub selected_connectors: Vec<ConnectorKind>,
    pub consent_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccountContext {
    pub subject_id: String,
    pub account_kind: AccountKind,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub policy_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConnectorConsent {
    pub consent_id: String,
    pub connector: ConnectorKind,
    pub state: ConsentState,
    pub scopes: Vec<String>,
    pub granted_at: Option<String>,
    pub expires_at: Option<String>,
    pub source_account_ref: String,
}

/// Opaque input to usefulness classification. Private message text remains in
/// the encrypted connector vault addressed by `encrypted_content_ref`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceItemCandidate {
    pub source_item_ref: String,
    pub connector: ConnectorKind,
    pub sender_class: SenderClass,
    pub received_at: String,
    pub thread_ref: String,
    pub encrypted_content_ref: String,
    pub content_sha256: String,
    pub has_direct_reply_request: bool,
    pub due_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UsefulnessDecision {
    pub decision_id: String,
    pub source_item_ref: String,
    pub disposition: UsefulnessDisposition,
    pub score: f32,
    pub reasons: Vec<UsefulnessReason>,
    pub model_ref: String,
    pub policy_version: String,
    pub evaluated_at: String,
    pub content_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SafeDeepLink {
    pub link_id: String,
    pub connector: ConnectorKind,
    pub target_url: String,
    pub decision_id: String,
    pub source_item_ref: String,
    pub expires_at: String,
    pub requires_reauthentication: bool,
    pub feed_fallback_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BriefingCard {
    pub card_id: String,
    pub kind: BriefingCardKind,
    pub priority: BriefingCardPriority,
    pub title: String,
    pub summary: String,
    pub source_label: String,
    pub observed_at: String,
    pub action_by: Option<String>,
    pub deep_link: Option<SafeDeepLink>,
    pub usefulness: Option<UsefulnessDecision>,
    pub uncertainty_notice: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MorningBriefing {
    pub schema: String,
    pub briefing_id: String,
    pub account: AccountContext,
    pub local_date: String,
    pub time_zone: String,
    pub generated_at: String,
    pub valid_until: String,
    pub cards: Vec<BriefingCard>,
    pub audio_url: Option<String>,
    pub audio_duration_seconds: Option<u32>,
    pub suppressed_item_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmbeddingDescriptor {
    pub vector_ref: String,
    pub tenant_id: String,
    pub source_item_ref: String,
    pub model_ref: String,
    pub dimensions: u32,
    pub content_sha256: String,
    pub retention_class: RetentionClass,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CorrelationFinding {
    pub finding_id: String,
    pub tenant_id: String,
    pub feature: String,
    pub outcome: String,
    pub coefficient: f64,
    pub p_value: f64,
    pub confidence_low: f64,
    pub confidence_high: f64,
    pub sample_size: u32,
    pub correction: MultipleTestingCorrection,
    pub causal_claim_allowed: bool,
    pub computed_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealtimeEnvelope {
    pub schema: String,
    pub event_id: String,
    pub tenant_id: String,
    pub subject_id: String,
    pub sequence: u64,
    pub transport: RealtimeTransport,
    pub resume_token: String,
    pub event_type: String,
    pub payload_json: String,
    pub emitted_at: String,
    pub ack_required: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatSession {
    pub session_id: String,
    pub tenant_id: String,
    pub audience: ChatAudience,
    pub shared_auth_subject: String,
    pub allowed_search_scopes: Vec<String>,
    pub opened_at: String,
    pub expires_at: String,
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

/// Tenant-scoped briefing persistence and retrieval boundary.
pub trait BriefingRepository: Send + Sync {
    type Error: Send + Sync + 'static;

    fn latest(
        &self,
        account: &AccountContext,
    ) -> impl Future<Output = Result<Option<MorningBriefing>, Self::Error>> + Send;
}

/// Classifier boundary. The implementation may inspect the encrypted content
/// under a short-lived, audited grant; consumers receive only the decision.
pub trait MessageUsefulnessClassifier: Send + Sync {
    type Error: Send + Sync + 'static;

    fn classify(
        &self,
        account: &AccountContext,
        candidate: SourceItemCandidate,
    ) -> impl Future<Output = Result<UsefulnessDecision, Self::Error>> + Send;
}

/// Realtime delivery boundary shared by WebSocket, persistent TLS/TCP, and
/// NATS implementations. Implementations must preserve sequence and tenant.
pub trait BriefingRealtimeTransport: Send + Sync {
    type Error: Send + Sync + 'static;

    fn publish(
        &self,
        event: RealtimeEnvelope,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
