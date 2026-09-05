import "@typespec/http";
import "@typespec/openapi3";

using TypeSpec.Http;

@service(#{ title: "Happy Wakey Morning Briefing API" })
@server("https://api.hawky.pro", "Production")
namespace HappyWakey.Briefing.V1;

alias AccountKind = "individual" | "organization_member";
alias ConnectorKind =
  | "email"
  | "whatsapp"
  | "linkedin"
  | "x_dm"
  | "slack"
  | "teams"
  | "calendar"
  | "weather"
  | "flights"
  | "markets"
  | "crm";
alias ConsentState = "pending" | "granted" | "revoked" | "expired";
alias UsefulnessDisposition = "useful" | "not_useful" | "needs_review";
alias UsefulnessReason =
  | "vip_sender"
  | "reply_requested"
  | "blocking_others"
  | "time_sensitive"
  | "travel_disruption"
  | "security_risk"
  | "financial_impact"
  | "customer_escalation"
  | "low_signal"
  | "promotional"
  | "unknown";
alias CardKind =
  | "this_day_in_history"
  | "useful_message"
  | "email_bottleneck"
  | "team_bottleneck"
  | "calendar"
  | "weather"
  | "extended_outlook"
  | "flight"
  | "market"
  | "kpi"
  | "task"
  | "news"
  | "audio_briefing";
alias CardPriority = "critical" | "high" | "normal" | "low";
alias RealtimeTransport = "websocket" | "tls_tcp" | "nats";
alias ChatAudience =
  | "sales_visitor"
  | "customer_support"
  | "organization_admin"
  | "internal_operator"
  | "owner";

model OnboardingIntent {
  @minLength(1) @maxLength(128) requestId: string;
  accountKind: AccountKind;
  @minLength(1) @maxLength(160) organizationName?: string;
  @minValue(1) @maxValue(100000) requestedSeatCount?: uint32;
  @minLength(1) @maxLength(128) timeZone: string;
  morningWindowStart: plainTime;
  @maxItems(16) selectedConnectors: ConnectorKind[];
  @minLength(1) @maxLength(128) consentVersion: string;
}

model AccountContext {
  @minLength(1) @maxLength(128) subjectId: string;
  accountKind: AccountKind;
  @minLength(1) @maxLength(128) tenantId: string;
  organizationId?: string;
  @minLength(1) @maxLength(128) policyVersion: string;
}

model ConnectorConsent {
  @minLength(1) @maxLength(128) consentId: string;
  connector: ConnectorKind;
  state: ConsentState;
  @maxItems(32) scopes: string[];
  grantedAt?: utcDateTime;
  expiresAt?: utcDateTime;
  @minLength(1) @maxLength(128) sourceAccountRef: string;
}

model SourceItemCandidate {
  @minLength(1) @maxLength(128) sourceItemRef: string;
  connector: ConnectorKind;
  senderClass: "vip" | "known" | "unknown";
  receivedAt: utcDateTime;
  @minLength(1) @maxLength(128) threadRef: string;
  @minLength(1) @maxLength(256) encryptedContentRef: string;
  contentSha256: string;
  hasDirectReplyRequest: boolean;
  dueAt?: utcDateTime;
}

model UsefulnessDecision {
  @minLength(1) @maxLength(128) decisionId: string;
  @minLength(1) @maxLength(128) sourceItemRef: string;
  disposition: UsefulnessDisposition;
  @minValue(0) @maxValue(1) score: float32;
  @minItems(1) @maxItems(8) reasons: UsefulnessReason[];
  @minLength(1) @maxLength(128) modelRef: string;
  @minLength(1) @maxLength(128) policyVersion: string;
  evaluatedAt: utcDateTime;
  contentSha256: string;
}

model SafeDeepLink {
  @minLength(1) @maxLength(128) linkId: string;
  connector: ConnectorKind;
  targetUrl: url;
  @minLength(1) @maxLength(128) decisionId: string;
  @minLength(1) @maxLength(128) sourceItemRef: string;
  expiresAt: utcDateTime;
  requiresReauthentication: boolean;
  feedFallbackAllowed: false;
}

model BriefingCard {
  @minLength(1) @maxLength(128) cardId: string;
  kind: CardKind;
  priority: CardPriority;
  @minLength(1) @maxLength(160) title: string;
  @minLength(1) @maxLength(1200) summary: string;
  sourceLabel: string;
  observedAt: utcDateTime;
  actionBy?: utcDateTime;
  deepLink?: SafeDeepLink;
  usefulness?: UsefulnessDecision;
  uncertaintyNotice?: string;
}

model MorningBriefing {
  schema: "happy-wakey.morning-briefing.v1";
  @minLength(1) @maxLength(128) briefingId: string;
  account: AccountContext;
  localDate: plainDate;
  timeZone: string;
  generatedAt: utcDateTime;
  validUntil: utcDateTime;
  @maxItems(64) cards: BriefingCard[];
  audioUrl?: url;
  @minValue(0) @maxValue(3600) audioDurationSeconds?: uint32;
  suppressedItemCount: uint32;
}

model EmbeddingDescriptor {
  @minLength(1) @maxLength(128) vectorRef: string;
  @minLength(1) @maxLength(128) tenantId: string;
  @minLength(1) @maxLength(128) sourceItemRef: string;
  @minLength(1) @maxLength(128) modelRef: string;
  @minValue(1) @maxValue(4100) dimensions: uint32;
  contentSha256: string;
  retentionClass: "ephemeral" | "standard" | "legal_hold";
  createdAt: utcDateTime;
}

model CorrelationFinding {
  @minLength(1) @maxLength(128) findingId: string;
  @minLength(1) @maxLength(128) tenantId: string;
  feature: string;
  outcome: string;
  @minValue(-1) @maxValue(1) coefficient: float64;
  @minValue(0) @maxValue(1) pValue: float64;
  confidenceLow: float64;
  confidenceHigh: float64;
  @minValue(3) sampleSize: uint32;
  correction: "none" | "bonferroni" | "benjamini_hochberg";
  causalClaimAllowed: false;
  computedAt: utcDateTime;
}

model RealtimeEnvelope {
  schema: "happy-wakey.realtime.v1";
  @minLength(1) @maxLength(128) eventId: string;
  @minLength(1) @maxLength(128) tenantId: string;
  @minLength(1) @maxLength(128) subjectId: string;
  sequence: uint64;
  transport: RealtimeTransport;
  @minLength(1) @maxLength(256) resumeToken: string;
  eventType: string;
  payloadJson: string;
  emittedAt: utcDateTime;
  ackRequired: boolean;
}

model ChatSession {
  @minLength(1) @maxLength(128) sessionId: string;
  @minLength(1) @maxLength(128) tenantId: string;
  audience: ChatAudience;
  @minLength(1) @maxLength(128) sharedAuthSubject: string;
  @maxItems(32) allowedSearchScopes: string[];
  openedAt: utcDateTime;
  expiresAt: utcDateTime;
}

@route("/v1/briefings/morning")
@get
op getMorningBriefing(@header authorization: string): MorningBriefing;

@route("/v1/briefings/{briefingId}/stream")
@get
op getBriefingStream(
  @path briefingId: string,
  @header authorization: string,
): RealtimeEnvelope;
