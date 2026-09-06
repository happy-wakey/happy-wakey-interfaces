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
    Briefing,
    DayPlan,
    Tasks,
    Habits,
    Focus,
    Sleep,
    Biometrics,
    Inbox,
    Messages,
    Commute,
    Portfolio,
    ProviderSync,
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

// ---------------------------------------------------------------------------
// Morning dashboard composition contracts.
//
// `MorningBriefing` above is the delivery representation: the cards a client
// renders, with their usefulness decisions and safe deep links. The types below
// are the layer beneath it — the domains a briefing is composed FROM, and the
// composition trace that records what each contributed. There is deliberately
// one delivery shape and one composition shape, not two briefings.
//
// These types mirror the schemas of the same name. Two invariants run through
// all of them and are the reason several fields exist at all.
//
// First, absence is data. A section, account or platform that could not be read
// says so — `SectionState`, `ProviderStatus` and `MessageAccessLevel` all carry
// a value for "this is not readable here" — because a brief that silently drops
// what it could not assemble is indistinguishable to the reader from a brief
// that had nothing to say.
//
// Second, authorization travels with the payload. `ContentClass` and
// `FeedClass` record what the upstream licence and consent actually granted, so
// a delayed quote cannot be rendered as live and a header-only mail scope
// cannot grow a body preview downstream.
// ---------------------------------------------------------------------------

/// An upstream source of dashboard data.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    GoogleCalendar,
    MicrosoftGraphCalendar,
    Caldav,
    IcsFeed,
    Gmail,
    MicrosoftGraphMail,
    Imap,
    Slack,
    Telegram,
    AppleHealthkit,
    AndroidHealthConnect,
    Oura,
    Whoop,
    GoogleHealth,
    Garmin,
    Withings,
    HealthAggregator,
    Alpaca,
    Finnhub,
    Massive,
    Snaptrade,
    OpenMeteo,
    Openweather,
    Todoist,
    Ticktick,
    Linear,
    Jira,
    Asana,
    Github,
    Notion,
    Newsapi,
    MapsCommute,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDomain {
    Calendar,
    Mail,
    Messaging,
    Health,
    Markets,
    Environment,
    Tasks,
    News,
    Commute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthKind {
    Oauth2AuthorizationCode,
    Oauth2Pkce,
    Oauth2Device,
    Oauth2ClientCredentials,
    ApiKey,
    AppPassword,
    PersonalAccessToken,
    OnDevicePermission,
    PublicUnauthenticated,
}

/// A single kind of data a connection may produce. Absence is authoritative:
/// a capability the owner did not grant must never be synthesized downstream.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCapability {
    CalendarEventsRead,
    CalendarEventsWrite,
    CalendarFreebusy,
    MailMetadataRead,
    MailBodyRead,
    MailWrite,
    MessageThreadsRead,
    MessageUnreadCount,
    SleepSessions,
    SleepStages,
    ReadinessScore,
    RecoveryScore,
    SleepDebt,
    HeartRate,
    HeartRateVariability,
    RestingHeartRate,
    RespiratoryRate,
    BloodOxygen,
    BodyTemperature,
    Steps,
    Workouts,
    EnergyExpenditure,
    CardioFitness,
    StressLoad,
    PortfolioHoldings,
    Quotes,
    EarningsCalendar,
    MarketNews,
    WeatherCurrent,
    WeatherForecast,
    WeatherAlerts,
    AirQuality,
    Pollen,
    UvIndex,
    Daylight,
    TasksRead,
    TasksWrite,
    NewsHeadlines,
    CommuteEta,
}

/// How much of the upstream payload a connection is authorized to read.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContentClass {
    None,
    CountsOnly,
    MetadataOnly,
    FullContent,
}

/// Timeliness the upstream licence grants. Never widened client side.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FeedClass {
    RealTime,
    Delayed,
    EndOfDay,
    OnSync,
    Unknown,
}

/// `UnsupportedPlatform` and `PolicyUnavailable` are terminal states, not
/// failures: the first records a capability the running platform cannot offer,
/// the second a source no third party may read at any tier. Neither is retried.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStatus {
    NotConnected,
    PendingConsent,
    Connected,
    Degraded,
    RateLimited,
    Expired,
    Revoked,
    UnsupportedPlatform,
    PolicyUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncFailure {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub occurred_at: String,
    pub retry_after_seconds: Option<u32>,
}

/// The upstream quota a connection must stay inside, so the scheduler can pace
/// itself rather than discover the limit as an error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RateBudget {
    pub requests: u32,
    pub window_seconds: u32,
    pub consumed: u32,
    pub resets_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProviderConnection {
    pub id: String,
    pub provider: Provider,
    pub domain: ProviderDomain,
    pub display_name: String,
    pub auth_kind: AuthKind,
    pub content_class: ContentClass,
    pub feed_class: FeedClass,
    pub requested_capabilities: Vec<ProviderCapability>,
    /// Always a subset of `requested_capabilities`. Providers with per-scope
    /// consent routinely grant less than was asked for; read this list.
    pub granted_capabilities: Vec<ProviderCapability>,
    pub status: ProviderStatus,
    /// Opaque handle into the secret boundary. Never a token or a key.
    pub credential_ref: Option<String>,
    pub last_sync_at: Option<String>,
    pub next_sync_after: Option<String>,
    pub last_failure: Option<SyncFailure>,
    pub rate_budget: Option<RateBudget>,
    pub generation: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConnectProviderRequest {
    pub transition_id: String,
    pub provider: Provider,
    pub display_name: String,
    pub requested_capabilities: Vec<ProviderCapability>,
    pub content_class: ContentClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementBasis {
    Wearable,
    Phone,
    Modeled,
    Manual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DataConfidence {
    High,
    Medium,
    Low,
    InsufficientData,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SleepStageMinutes {
    pub awake: u16,
    pub light: u16,
    pub deep: u16,
    pub rem: u16,
    /// Minutes scored as asleep without a stage. Must not be folded into
    /// `light` to make a chart look complete.
    pub unspecified: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OvernightVitals {
    pub resting_heart_rate_bpm: Option<f32>,
    pub heart_rate_variability_ms: Option<f32>,
    pub respiratory_rate_bpm: Option<f32>,
    pub blood_oxygen_percent: Option<f32>,
    pub skin_temperature_deviation_c: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SleepSummary {
    pub id: String,
    /// Civil date of the evening the owner went to bed, in `time_zone`.
    pub night_of: String,
    pub time_zone: String,
    pub source_connection_id: String,
    pub source_provider: Provider,
    pub measurement_basis: MeasurementBasis,
    pub confidence: DataConfidence,
    pub bedtime_at: String,
    pub wake_at: String,
    pub time_in_bed_minutes: u16,
    pub asleep_minutes: u16,
    pub sleep_latency_minutes: Option<u16>,
    pub awakenings: Option<u16>,
    pub efficiency_percent: Option<f32>,
    pub stage_minutes: Option<SleepStageMinutes>,
    pub overnight_vitals: OvernightVitals,
    /// The provider's own composite. Never back-filled with a local formula
    /// presented as the provider's number.
    pub sleep_score: Option<u8>,
    pub sleep_debt_minutes: Option<i32>,
    pub regularity_percent: Option<f32>,
    pub recommended_bedtime_local: Option<String>,
    pub generation: u64,
    pub recorded_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BiometricMetric {
    RestingHeartRate,
    HeartRateVariability,
    RespiratoryRate,
    BloodOxygen,
    BodyTemperature,
    SleepDuration,
    SleepEfficiency,
    Steps,
    ActiveEnergy,
    CardioFitness,
    StressLoad,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BaselineDirection {
    AboveBaseline,
    BelowBaseline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnomalySeverity {
    Notable,
    Marked,
    Extreme,
}

/// A metric outside the owner's own trailing distribution. This is a statement
/// about a personal baseline and never a clinical finding; the contract carries
/// no diagnosis field and surfaces must not render one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BiometricAnomaly {
    pub metric: BiometricMetric,
    pub direction: BaselineDirection,
    pub severity: AnomalySeverity,
    pub observed: f64,
    pub baseline_mean: f64,
    pub z_score: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BiometricSummary {
    pub id: String,
    pub day: String,
    pub time_zone: String,
    pub source_connection_id: String,
    pub source_provider: Provider,
    pub measurement_basis: MeasurementBasis,
    pub confidence: DataConfidence,
    /// Days of the owner's own history behind the baselines. Below the
    /// calibration window, anomalies are suppressed rather than widened.
    pub baseline_days: u16,
    pub readiness_score: Option<u8>,
    pub recovery_percent: Option<f32>,
    pub strain_load: Option<f32>,
    pub resting_heart_rate_bpm: Option<f32>,
    pub heart_rate_variability_ms: Option<f32>,
    pub respiratory_rate_bpm: Option<f32>,
    pub blood_oxygen_percent: Option<f32>,
    pub skin_temperature_deviation_c: Option<f32>,
    pub steps: Option<u32>,
    pub active_energy_kcal: Option<f32>,
    pub workout_minutes: Option<u16>,
    pub cardio_fitness_vo2max: Option<f32>,
    pub anomalies: Vec<BiometricAnomaly>,
    pub generation: u64,
    pub recorded_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlanBlockKind {
    CalendarEvent,
    Task,
    Habit,
    Focus,
    Break,
    Meal,
    Commute,
    Buffer,
    WindDown,
    Sleep,
}

/// `ExternalMirror` blocks are read-only copies of an upstream event: the
/// planner schedules around them and never moves them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlanBlockOrigin {
    ExternalMirror,
    HappyWakey,
    Owner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnergyPhase {
    Grogginess,
    Rising,
    Peak,
    Dip,
    SecondWind,
    WindDown,
    MelatoninWindow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlanConflictKind {
    Overlap,
    OverCapacity,
    InsufficientTravelTime,
    OutsideWorkingHours,
    PastDeadline,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlanBlock {
    pub id: String,
    pub kind: PlanBlockKind,
    pub origin: PlanBlockOrigin,
    pub title: String,
    pub starts_at: String,
    pub ends_at: String,
    pub is_flexible: bool,
    pub is_all_day: bool,
    pub priority: u8,
    pub linked_entity_id: Option<String>,
    pub source_connection_id: Option<String>,
    pub location_label: Option<String>,
    pub join_url: Option<String>,
    /// How well the placement matches the predicted curve. `None` when no
    /// circadian signal was available; an absent signal is not a neutral score.
    pub energy_fit: Option<u8>,
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DayCapacity {
    pub available_minutes: u16,
    pub committed_minutes: u16,
    pub planned_minutes: u16,
    /// Planned minutes that do not fit. Surfaced, never absorbed silently.
    pub overcommitted_minutes: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EnergyPoint {
    pub starts_at: String,
    pub phase: EnergyPhase,
    pub level: u8,
    pub basis: MeasurementBasis,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlanConflict {
    pub kind: PlanConflictKind,
    pub block_ids: Vec<String>,
    pub detail: String,
}

/// An item carried from an earlier day that the owner has not re-decided.
/// Silent carry-forward is the failure mode this record exists to prevent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlanRollover {
    pub entity_id: String,
    pub title: String,
    pub original_day: String,
    pub times_rolled: u32,
    pub decided: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DayPlan {
    pub id: String,
    pub day: String,
    pub time_zone: String,
    pub working_hours_start_local: String,
    pub working_hours_end_local: String,
    pub blocks: Vec<PlanBlock>,
    pub capacity: DayCapacity,
    pub energy_curve: Vec<EnergyPoint>,
    pub conflicts: Vec<PlanConflict>,
    pub rollovers: Vec<PlanRollover>,
    /// `None` means the plan is machine-proposed and not yet confirmed.
    pub planned_at: Option<String>,
    pub generation: u64,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HabitCadence {
    Daily,
    Weekly,
    Monthly,
    WeekdaySet,
    TimesPerWeek,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HabitEntryState {
    Pending,
    Completed,
    Skipped,
    Missed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Habit {
    pub id: String,
    pub name: String,
    pub cadence: HabitCadence,
    pub target_per_period: u8,
    pub weekdays: Vec<u8>,
    pub preferred_window_start_local: String,
    pub preferred_window_end_local: String,
    pub duration_minutes: u16,
    pub is_flexible: bool,
    pub enabled: bool,
    pub time_zone: String,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub tags: Vec<String>,
    pub generation: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HabitEntry {
    pub id: String,
    pub habit_id: String,
    pub day: String,
    pub state: HabitEntryState,
    /// Present exactly when `state` is `Completed`; the two are one fact.
    pub completed_at: Option<String>,
    pub block_id: Option<String>,
    pub generation: u64,
    pub updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateHabitRequest {
    pub transition_id: String,
    pub name: String,
    pub cadence: HabitCadence,
    pub target_per_period: u8,
    pub weekdays: Vec<u8>,
    pub preferred_window_start_local: String,
    pub preferred_window_end_local: String,
    pub duration_minutes: u16,
    pub is_flexible: bool,
    pub enabled: bool,
    pub time_zone: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RecordHabitEntryRequest {
    pub transition_id: String,
    pub expected_generation: u64,
    pub day: String,
    pub state: HabitEntryState,
    pub client_time: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboxImportanceReason {
    VipSender,
    AddressedDirectly,
    ThreadYouStarted,
    AwaitingYourReply,
    MentionsDeadline,
    CalendarInvite,
    FrequentCorrespondent,
    FirstTimeSender,
    FlaggedByProvider,
    BulkOrList,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InboxDigestItem {
    /// Provider-opaque identifier used to deep link back. Never an address.
    pub message_ref: String,
    pub thread_ref: String,
    pub sender_display: String,
    pub sender_domain: String,
    pub subject: String,
    pub received_at: String,
    pub is_unread: bool,
    pub is_calendar_invite: bool,
    pub has_attachment: bool,
    pub labels: Vec<String>,
    pub importance: u8,
    pub importance_reasons: Vec<InboxImportanceReason>,
    /// Permitted only when the account's `content_class` is `FullContent`.
    pub snippet: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InboxAccountDigest {
    pub source_connection_id: String,
    pub provider: Provider,
    pub account_label: String,
    pub content_class: ContentClass,
    pub status: ProviderStatus,
    pub unread_total: u32,
    pub unread_since_window_start: u32,
    pub items: Vec<InboxDigestItem>,
    pub truncated: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InboxDigest {
    pub schema: String,
    pub generated_at: String,
    pub window_start: String,
    pub window_end: String,
    pub accounts: Vec<InboxAccountDigest>,
    /// Domains rather than addresses: senders are ranked without holding the
    /// owner's address book.
    pub vip_domains: Vec<String>,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessagePlatform {
    Slack,
    Telegram,
    Discord,
    Whatsapp,
    Instagram,
    Linkedin,
    Imessage,
    X,
    OsNotifications,
}

/// What a third party may actually read from a platform. `ThrottledRead` is
/// sanctioned but capped hard enough that completeness is not guaranteed;
/// `PolicyUnavailable` means no read exists at any tier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageAccessLevel {
    FullRead,
    ThrottledRead,
    CountOnly,
    PolicyUnavailable,
    PlatformUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MessageThread {
    pub thread_ref: String,
    pub counterpart_display: String,
    pub is_group: bool,
    pub unread_count: u32,
    pub last_message_at: String,
    /// Permitted only at `FullRead` or `ThrottledRead`.
    pub preview: Option<String>,
    pub mentions_owner: bool,
    pub deep_link: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MessagePlatformDigest {
    pub platform: MessagePlatform,
    pub access_level: MessageAccessLevel,
    pub source_connection_id: Option<String>,
    pub status: ProviderStatus,
    pub unread_total: Option<u32>,
    pub threads: Vec<MessageThread>,
    pub truncated: bool,
    /// Why this list may be short: a rate cap, a consent gap, or a policy
    /// prohibition. Required so the surface can say what it does not know.
    pub completeness_note: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MessageDigest {
    pub schema: String,
    pub generated_at: String,
    pub window_start: String,
    pub window_end: String,
    pub platforms: Vec<MessagePlatformDigest>,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarketPhase {
    PreMarket,
    Open,
    AfterHours,
    Closed,
    Holiday,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PriceAlertDirection {
    Above,
    Below,
    PercentMove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarketCalendarKind {
    Earnings,
    Dividend,
    Split,
    EconomicRelease,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Quote {
    pub symbol: String,
    pub display_name: String,
    pub currency: String,
    pub last_price: f64,
    pub previous_close: Option<f64>,
    pub change_absolute: Option<f64>,
    pub change_percent: Option<f64>,
    /// When the price was true at the venue, not when it was fetched.
    pub struck_at: String,
    pub feed_class: FeedClass,
    pub source_connection_id: String,
    pub source_provider: Provider,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Holding {
    /// Opaque brokerage account handle. Never an account number.
    pub account_ref: String,
    pub symbol: String,
    pub quantity: f64,
    pub currency: String,
    pub market_value: Option<f64>,
    pub cost_basis: Option<f64>,
    pub unrealized_change: Option<f64>,
    pub as_of: String,
    pub source_connection_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PriceAlert {
    pub id: String,
    pub symbol: String,
    pub direction: PriceAlertDirection,
    pub threshold: f64,
    pub triggered_at: String,
    pub observed_price: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MarketCalendarEvent {
    pub kind: MarketCalendarKind,
    pub symbol: Option<String>,
    pub headline: String,
    pub scheduled_for: String,
    pub is_estimated: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MarketWatch {
    pub schema: String,
    pub generated_at: String,
    pub market_phase: MarketPhase,
    pub quotes: Vec<Quote>,
    pub holdings: Vec<Holding>,
    pub triggered_alerts: Vec<PriceAlert>,
    pub calendar: Vec<MarketCalendarEvent>,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WeatherCondition {
    Clear,
    PartlyCloudy,
    Cloudy,
    Fog,
    Drizzle,
    Rain,
    FreezingRain,
    Snow,
    Sleet,
    Thunderstorm,
    Hail,
    Wind,
    Dust,
    Smoke,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AirQualityScale {
    UsAqi,
    EuAqi,
    UkDaqi,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AirQualityCategory {
    Good,
    Moderate,
    UnhealthySensitive,
    Unhealthy,
    VeryUnhealthy,
    Hazardous,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Pollutant {
    Pm25,
    Pm10,
    Ozone,
    No2,
    So2,
    Co,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PollenKind {
    Tree,
    Grass,
    Weed,
    Mold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PollenLevel {
    None,
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlertSeverity {
    Advisory,
    Watch,
    Warning,
    Emergency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommuteMode {
    Drive,
    Transit,
    Walk,
    Cycle,
}

/// City-level, deliberately. A dashboard needs a sunrise and a forecast, not a
/// street address, so coordinates are rounded to roughly a kilometre.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CoarseLocation {
    pub label: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WeatherConditions {
    pub condition: WeatherCondition,
    pub temperature_c: f32,
    pub feels_like_c: Option<f32>,
    pub relative_humidity_percent: Option<f32>,
    pub wind_speed_kph: Option<f32>,
    pub wind_gust_kph: Option<f32>,
    pub precipitation_mm: Option<f32>,
    pub precipitation_probability_percent: Option<f32>,
    pub cloud_cover_percent: Option<f32>,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HourlyForecastPoint {
    pub starts_at: String,
    pub condition: WeatherCondition,
    pub temperature_c: f32,
    pub precipitation_probability_percent: Option<f32>,
}

/// Polar day and polar night are expressed as absent sunrise and sunset rather
/// than a fabricated time.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DaylightWindow {
    pub civil_dawn_at: Option<String>,
    pub sunrise_at: Option<String>,
    pub sunset_at: Option<String>,
    pub civil_dusk_at: Option<String>,
    pub daylight_minutes: Option<u16>,
    pub daylight_change_minutes: Option<i16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AirQuality {
    pub index: f32,
    pub scale: AirQualityScale,
    pub category: AirQualityCategory,
    pub dominant_pollutant: Option<Pollutant>,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PollenReading {
    pub kind: PollenKind,
    pub level: PollenLevel,
    pub index: Option<f32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WeatherAlert {
    pub severity: AlertSeverity,
    pub headline: String,
    pub issuer: String,
    pub effective_at: String,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommuteEstimate {
    pub origin_label: String,
    pub destination_label: String,
    pub mode: CommuteMode,
    pub duration_minutes: u16,
    pub typical_minutes: Option<u16>,
    pub delay_minutes: Option<i16>,
    pub leave_by_at: Option<String>,
    pub for_block_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentBrief {
    pub schema: String,
    pub generated_at: String,
    pub location: CoarseLocation,
    pub current: WeatherConditions,
    pub hourly: Vec<HourlyForecastPoint>,
    pub high_temperature_c: Option<f32>,
    pub low_temperature_c: Option<f32>,
    pub daylight: DaylightWindow,
    pub air_quality: Option<AirQuality>,
    pub uv_index_max: Option<f32>,
    pub pollen: Vec<PollenReading>,
    pub alerts: Vec<WeatherAlert>,
    pub commutes: Vec<CommuteEstimate>,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompositionSectionKind {
    Schedule,
    Tasks,
    Habits,
    FocusWindows,
    Sleep,
    Biometrics,
    Anomalies,
    Inbox,
    Messages,
    Markets,
    Environment,
    Commute,
    News,
}

/// Only `Ready` may carry items. Every other value is a legible gap, which is
/// the whole point: a brief that silently drops what it could not assemble
/// reads to the owner as a brief with nothing to say.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompositionSectionState {
    Ready,
    Empty,
    Degraded,
    Stale,
    NotConnected,
    Unavailable,
    Failed,
}

/// Why an item is in the brief. Carried in the payload so the interface can
/// answer "why am I seeing this" without re-deriving the ranking.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceReason {
    NextUp,
    StartsSoon,
    DeadlineToday,
    Overdue,
    Conflict,
    VipSender,
    AwaitingYourReply,
    OutsideBaseline,
    StreakAtRisk,
    ThresholdCrossed,
    UnusualDelay,
    SevereAlert,
    FirstOfDay,
    OwnerPinned,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SectionProvenance {
    pub source_connection_id: Option<String>,
    pub provider: Provider,
    pub content_class: ContentClass,
    pub feed_class: FeedClass,
    pub fetched_at: String,
    pub staleness_seconds: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CompositionItem {
    pub id: String,
    pub rank: u16,
    pub headline: String,
    pub detail: Option<String>,
    pub occurs_at: Option<String>,
    pub reasons: Vec<SurfaceReason>,
    pub entity_ref: Option<String>,
    pub deep_link: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CompositionSection {
    pub kind: CompositionSectionKind,
    pub rank: u8,
    pub state: CompositionSectionState,
    /// Present for every state, including the states that carry no items, so a
    /// gap is still legible.
    pub summary: String,
    pub items: Vec<CompositionItem>,
    pub provenance: Vec<SectionProvenance>,
    /// Required whenever `state` is not `Ready`.
    pub state_detail: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BriefingComposition {
    pub id: String,
    pub schema: String,
    pub for_day: String,
    pub time_zone: String,
    pub generated_at: String,
    pub delivered_at: Option<String>,
    pub headline: String,
    pub sections: Vec<CompositionSection>,
    /// Sections the owner switched off, listed rather than omitted so the brief
    /// can distinguish "you turned this off" from "this broke".
    pub sections_withheld: Vec<CompositionSectionKind>,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DashboardModule {
    BriefingComposition,
    Schedule,
    DayPlan,
    Tasks,
    Habits,
    Focus,
    Alarms,
    Sleep,
    Biometrics,
    Inbox,
    Messages,
    Markets,
    Environment,
    Commute,
    News,
    Bookmarks,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModuleSize {
    Compact,
    Standard,
    Wide,
    Full,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutContext {
    Morning,
    Workday,
    Evening,
    Weekend,
    Travel,
    Always,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutTheme {
    System,
    Light,
    Dark,
    NightShift,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutDensity {
    Comfortable,
    Compact,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModulePlacement {
    pub module: DashboardModule,
    pub visible: bool,
    pub position: u8,
    pub size: ModuleSize,
    pub contexts: Vec<LayoutContext>,
    /// A floor the client must respect, not a target it may beat: every module
    /// maps onto an upstream quota.
    pub refresh_interval_seconds: u32,
    pub collapsed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModuleLayout {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub theme: LayoutTheme,
    pub accent: String,
    pub density: LayoutDensity,
    /// `None` leaves the brief on demand rather than scheduled.
    pub briefing_delivery_local: Option<String>,
    pub quiet_hours_start_local: Option<String>,
    pub quiet_hours_end_local: Option<String>,
    pub modules: Vec<ModulePlacement>,
    pub generation: u64,
    pub updated_at: String,
}

/// Read boundary for the dashboard domains, implemented by the API service.
///
/// Every method is owner-scoped by a Shared Auth identity resolved at the
/// service boundary. A day with no data returns `None` rather than a zeroed
/// record, so a caller cannot mistake "not measured" for "measured as zero".
pub trait DashboardRepository: Send + Sync {
    type Error: Send + Sync + 'static;

    fn list_provider_connections(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<Vec<ProviderConnection>, Self::Error>> + Send;

    fn connect_provider(
        &self,
        owner_id: &str,
        request: ConnectProviderRequest,
    ) -> impl Future<Output = Result<ProviderConnection, Self::Error>> + Send;

    fn disconnect_provider(
        &self,
        owner_id: &str,
        connection_id: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn get_briefing_composition(
        &self,
        owner_id: &str,
        day: &str,
    ) -> impl Future<Output = Result<Option<BriefingComposition>, Self::Error>> + Send;

    fn get_day_plan(
        &self,
        owner_id: &str,
        day: &str,
    ) -> impl Future<Output = Result<Option<DayPlan>, Self::Error>> + Send;

    fn get_sleep_summary(
        &self,
        owner_id: &str,
        night_of: &str,
    ) -> impl Future<Output = Result<Option<SleepSummary>, Self::Error>> + Send;

    fn get_biometric_summary(
        &self,
        owner_id: &str,
        day: &str,
    ) -> impl Future<Output = Result<Option<BiometricSummary>, Self::Error>> + Send;

    fn list_habits(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<Vec<Habit>, Self::Error>> + Send;

    fn create_habit(
        &self,
        owner_id: &str,
        request: CreateHabitRequest,
    ) -> impl Future<Output = Result<Habit, Self::Error>> + Send;

    fn record_habit_entry(
        &self,
        owner_id: &str,
        habit_id: &str,
        request: RecordHabitEntryRequest,
    ) -> impl Future<Output = Result<HabitEntry, Self::Error>> + Send;

    fn get_module_layout(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<Option<ModuleLayout>, Self::Error>> + Send;

    fn put_module_layout(
        &self,
        owner_id: &str,
        layout: ModuleLayout,
    ) -> impl Future<Output = Result<ModuleLayout, Self::Error>> + Send;
}

/// Composition boundary for the digests that are assembled from live upstream
/// reads rather than served from durable rows.
///
/// Implementations must return a value for every requested domain. A source
/// that could not be read is reported through its own status and access level,
/// never by omitting it from the result.
pub trait DigestComposer: Send + Sync {
    type Error: Send + Sync + 'static;

    fn compose_inbox_digest(
        &self,
        owner_id: &str,
        window_hours: u16,
    ) -> impl Future<Output = Result<InboxDigest, Self::Error>> + Send;

    fn compose_message_digest(
        &self,
        owner_id: &str,
        window_hours: u16,
    ) -> impl Future<Output = Result<MessageDigest, Self::Error>> + Send;

    fn compose_market_watch(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<MarketWatch, Self::Error>> + Send;

    fn compose_environment_brief(
        &self,
        owner_id: &str,
    ) -> impl Future<Output = Result<EnvironmentBrief, Self::Error>> + Send;
}
