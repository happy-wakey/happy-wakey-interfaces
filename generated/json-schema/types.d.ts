/* Generated from the independent JSON Schema authority. Do not edit. */

export interface OnboardingIntent {
  requestId: string;
  accountKind: "individual" | "organization_member";
  organizationName?: string;
  requestedSeatCount?: number;
  timeZone: string;
  morningWindowStart: string;
  selectedConnectors: Array<ConnectorKind>;
  consentVersion: string;
}

export interface AccountContext {
  subjectId: string;
  accountKind: "individual" | "organization_member";
  tenantId: string;
  organizationId?: string;
  policyVersion: string;
}

export interface ConnectorConsent {
  consentId: string;
  connector: ConnectorKind;
  state: "pending" | "granted" | "revoked" | "expired";
  scopes: Array<string>;
  grantedAt?: string;
  expiresAt?: string;
  sourceAccountRef: string;
}

export type ConnectorKind = "email" | "whatsapp" | "linkedin" | "x_dm" | "slack" | "teams" | "calendar" | "weather" | "flights" | "markets" | "crm";

export interface SourceItemCandidate {
  sourceItemRef: string;
  connector: ConnectorKind;
  senderClass: "vip" | "known" | "unknown";
  receivedAt: string;
  threadRef: string;
  encryptedContentRef: string;
  contentSha256: string;
  hasDirectReplyRequest: boolean;
  dueAt?: string;
}

export interface UsefulnessDecision {
  decisionId: string;
  sourceItemRef: string;
  disposition: "useful" | "not_useful" | "needs_review";
  score: number;
  reasons: Array<"vip_sender" | "reply_requested" | "blocking_others" | "time_sensitive" | "travel_disruption" | "security_risk" | "financial_impact" | "customer_escalation" | "low_signal" | "promotional" | "unknown">;
  modelRef: string;
  policyVersion: string;
  evaluatedAt: string;
  contentSha256: string;
}

export interface SafeDeepLink {
  linkId: string;
  connector: ConnectorKind;
  targetUrl: string;
  decisionId: string;
  sourceItemRef: string;
  expiresAt: string;
  requiresReauthentication: boolean;
  feedFallbackAllowed: false;
}

export interface BriefingCard {
  cardId: string;
  kind: "this_day_in_history" | "useful_message" | "email_bottleneck" | "team_bottleneck" | "calendar" | "weather" | "extended_outlook" | "flight" | "market" | "kpi" | "task" | "news" | "audio_briefing";
  priority: "critical" | "high" | "normal" | "low";
  title: string;
  summary: string;
  sourceLabel: string;
  observedAt: string;
  actionBy?: string;
  deepLink?: SafeDeepLink;
  usefulness?: UsefulnessDecision;
  uncertaintyNotice?: string;
}

export interface MorningBriefing {
  schema: "happy-wakey.morning-briefing.v1";
  briefingId: string;
  account: AccountContext;
  localDate: string;
  timeZone: string;
  generatedAt: string;
  validUntil: string;
  cards: Array<BriefingCard>;
  audioUrl?: string;
  audioDurationSeconds?: number;
  suppressedItemCount: number;
}

export interface EmbeddingDescriptor {
  vectorRef: string;
  tenantId: string;
  sourceItemRef: string;
  modelRef: string;
  dimensions: number;
  contentSha256: string;
  retentionClass: "ephemeral" | "standard" | "legal_hold";
  createdAt: string;
}

export interface CorrelationFinding {
  findingId: string;
  tenantId: string;
  feature: string;
  outcome: string;
  coefficient: number;
  pValue: number;
  confidenceLow: number;
  confidenceHigh: number;
  sampleSize: number;
  correction: "none" | "bonferroni" | "benjamini_hochberg";
  causalClaimAllowed: false;
  computedAt: string;
}

export interface RealtimeEnvelope {
  schema: "happy-wakey.realtime.v1";
  eventId: string;
  tenantId: string;
  subjectId: string;
  sequence: number;
  transport: "websocket" | "tls_tcp" | "nats";
  resumeToken: string;
  eventType: string;
  payloadJson: string;
  emittedAt: string;
  ackRequired: boolean;
}

export interface ChatSession {
  sessionId: string;
  tenantId: string;
  audience: "sales_visitor" | "customer_support" | "organization_admin" | "internal_operator" | "owner";
  sharedAuthSubject: string;
  allowedSearchScopes: Array<string>;
  openedAt: string;
  expiresAt: string;
}

export type MorningBriefingDocument = MorningBriefing;
