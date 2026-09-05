export interface paths {
    "/v1/briefings/{briefingId}/stream": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["getBriefingStream"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/v1/briefings/morning": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["getMorningBriefing"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        AccountContext: {
            /** @enum {string} */
            accountKind: "individual" | "organization_member";
            organizationId?: string;
            policyVersion: string;
            subjectId: string;
            tenantId: string;
        };
        BriefingCard: {
            /** Format: date-time */
            actionBy?: string;
            cardId: string;
            deepLink?: components["schemas"]["SafeDeepLink"];
            /** @enum {string} */
            kind: "this_day_in_history" | "useful_message" | "email_bottleneck" | "team_bottleneck" | "calendar" | "weather" | "extended_outlook" | "flight" | "market" | "kpi" | "task" | "news" | "audio_briefing";
            /** Format: date-time */
            observedAt: string;
            /** @enum {string} */
            priority: "critical" | "high" | "normal" | "low";
            sourceLabel: string;
            summary: string;
            title: string;
            uncertaintyNotice?: string;
            usefulness?: components["schemas"]["UsefulnessDecision"];
        };
        ChatSession: {
            allowedSearchScopes: string[];
            /** @enum {string} */
            audience: "sales_visitor" | "customer_support" | "organization_admin" | "internal_operator" | "owner";
            /** Format: date-time */
            expiresAt: string;
            /** Format: date-time */
            openedAt: string;
            sessionId: string;
            sharedAuthSubject: string;
            tenantId: string;
        };
        ConnectorConsent: {
            /** @enum {string} */
            connector: "email" | "whatsapp" | "linkedin" | "x_dm" | "slack" | "teams" | "calendar" | "weather" | "flights" | "markets" | "crm";
            consentId: string;
            /** Format: date-time */
            expiresAt?: string;
            /** Format: date-time */
            grantedAt?: string;
            scopes: string[];
            sourceAccountRef: string;
            /** @enum {string} */
            state: "pending" | "granted" | "revoked" | "expired";
        };
        CorrelationFinding: {
            /** @enum {boolean} */
            causalClaimAllowed: false;
            /** Format: double */
            coefficient: number;
            /** Format: date-time */
            computedAt: string;
            /** Format: double */
            confidenceHigh: number;
            /** Format: double */
            confidenceLow: number;
            /** @enum {string} */
            correction: "none" | "bonferroni" | "benjamini_hochberg";
            feature: string;
            findingId: string;
            outcome: string;
            /** Format: double */
            pValue: number;
            /** Format: uint32 */
            sampleSize: number;
            tenantId: string;
        };
        EmbeddingDescriptor: {
            contentSha256: string;
            /** Format: date-time */
            createdAt: string;
            /** Format: uint32 */
            dimensions: number;
            modelRef: string;
            /** @enum {string} */
            retentionClass: "ephemeral" | "standard" | "legal_hold";
            sourceItemRef: string;
            tenantId: string;
            vectorRef: string;
        };
        MorningBriefing: {
            account: components["schemas"]["AccountContext"];
            /** Format: uint32 */
            audioDurationSeconds?: number;
            /** Format: uri */
            audioUrl?: string;
            briefingId: string;
            cards: components["schemas"]["BriefingCard"][];
            /** Format: date-time */
            generatedAt: string;
            /** Format: date */
            localDate: string;
            /** @enum {string} */
            schema: "happy-wakey.morning-briefing.v1";
            /** Format: uint32 */
            suppressedItemCount: number;
            timeZone: string;
            /** Format: date-time */
            validUntil: string;
        };
        OnboardingIntent: {
            /** @enum {string} */
            accountKind: "individual" | "organization_member";
            consentVersion: string;
            /** Format: time */
            morningWindowStart: string;
            organizationName?: string;
            /** Format: uint32 */
            requestedSeatCount?: number;
            requestId: string;
            selectedConnectors: ("email" | "whatsapp" | "linkedin" | "x_dm" | "slack" | "teams" | "calendar" | "weather" | "flights" | "markets" | "crm")[];
            timeZone: string;
        };
        RealtimeEnvelope: {
            ackRequired: boolean;
            /** Format: date-time */
            emittedAt: string;
            eventId: string;
            eventType: string;
            payloadJson: string;
            resumeToken: string;
            /** @enum {string} */
            schema: "happy-wakey.realtime.v1";
            /** Format: uint64 */
            sequence: number;
            subjectId: string;
            tenantId: string;
            /** @enum {string} */
            transport: "websocket" | "tls_tcp" | "nats";
        };
        SafeDeepLink: {
            /** @enum {string} */
            connector: "email" | "whatsapp" | "linkedin" | "x_dm" | "slack" | "teams" | "calendar" | "weather" | "flights" | "markets" | "crm";
            decisionId: string;
            /** Format: date-time */
            expiresAt: string;
            /** @enum {boolean} */
            feedFallbackAllowed: false;
            linkId: string;
            requiresReauthentication: boolean;
            sourceItemRef: string;
            /** Format: uri */
            targetUrl: string;
        };
        SourceItemCandidate: {
            /** @enum {string} */
            connector: "email" | "whatsapp" | "linkedin" | "x_dm" | "slack" | "teams" | "calendar" | "weather" | "flights" | "markets" | "crm";
            contentSha256: string;
            /** Format: date-time */
            dueAt?: string;
            encryptedContentRef: string;
            hasDirectReplyRequest: boolean;
            /** Format: date-time */
            receivedAt: string;
            /** @enum {string} */
            senderClass: "vip" | "known" | "unknown";
            sourceItemRef: string;
            threadRef: string;
        };
        UsefulnessDecision: {
            contentSha256: string;
            decisionId: string;
            /** @enum {string} */
            disposition: "useful" | "not_useful" | "needs_review";
            /** Format: date-time */
            evaluatedAt: string;
            modelRef: string;
            policyVersion: string;
            reasons: ("vip_sender" | "reply_requested" | "blocking_others" | "time_sensitive" | "travel_disruption" | "security_risk" | "financial_impact" | "customer_escalation" | "low_signal" | "promotional" | "unknown")[];
            /** Format: float */
            score: number;
            sourceItemRef: string;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    getBriefingStream: {
        parameters: {
            query?: never;
            header: {
                authorization: string;
            };
            path: {
                briefingId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description The request has succeeded. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RealtimeEnvelope"];
                };
            };
        };
    };
    getMorningBriefing: {
        parameters: {
            query?: never;
            header: {
                authorization: string;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description The request has succeeded. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["MorningBriefing"];
                };
            };
        };
    };
}
