import "@typespec/protobuf";

using Protobuf;

@package
namespace HappyWakey.Briefing.V1;

enum BriefingCardKind {
  BRIEFING_CARD_KIND_UNSPECIFIED: 0,
  THIS_DAY_IN_HISTORY: 1,
  USEFUL_MESSAGE: 2,
  EMAIL_BOTTLENECK: 3,
  TEAM_BOTTLENECK: 4,
  CALENDAR: 5,
  WEATHER: 6,
  EXTENDED_OUTLOOK: 7,
  FLIGHT: 8,
  MARKET: 9,
  KPI: 10,
  TASK: 11,
  NEWS: 12,
  AUDIO_BRIEFING: 13,
}

enum BriefingTransport {
  BRIEFING_TRANSPORT_UNSPECIFIED: 0,
  WEBSOCKET: 1,
  TLS_TCP: 2,
  NATS: 3,
}

model BriefingStreamRequest {
  @field(1) briefingId: string;
  @field(2) tenantId: string;
  @field(3) subjectId: string;
  @field(4) resumeToken?: string;
}

model BriefingStreamEvent {
  @field(1) eventId: string;
  @field(2) sequence: uint64;
  @field(3) transport: BriefingTransport;
  @field(4) eventType: string;
  @field(5) payloadJson: string;
  @field(6) emittedAtRfc3339: string;
  @field(7) ackRequired: boolean;
}

model BriefingCardWire {
  @field(1) cardId: string;
  @field(2) kind: BriefingCardKind;
  @field(3) title: string;
  @field(4) summary: string;
  @field(5) sourceLabel: string;
  @field(6) observedAtRfc3339: string;
}

@Protobuf.service
interface BriefingRealtime {
  getBriefingStream(...BriefingStreamRequest): BriefingStreamEvent;
}
