-- PostgreSQL/Supabase overlay for sql/schema.sql.
-- The request transaction must SET LOCAL both settings after Shared Auth.
-- Empty or missing settings match no row, so every policy fails closed.

ALTER TABLE happy_wakey_tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_tenant_memberships ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_connector_consents ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_source_item_candidates ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_usefulness_decisions ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_safe_deep_links ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_morning_briefings ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_correlation_findings ENABLE ROW LEVEL SECURITY;
ALTER TABLE happy_wakey_chat_sessions ENABLE ROW LEVEL SECURITY;

CREATE POLICY happy_wakey_tenants_scope ON happy_wakey_tenants
  USING (id = current_setting('app.tenant_id', true));
CREATE POLICY happy_wakey_memberships_scope ON happy_wakey_tenant_memberships
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_consents_scope ON happy_wakey_connector_consents
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_candidates_scope ON happy_wakey_source_item_candidates
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_decisions_scope ON happy_wakey_usefulness_decisions
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_links_scope ON happy_wakey_safe_deep_links
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_briefings_scope ON happy_wakey_morning_briefings
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_embeddings_scope ON happy_wakey_embeddings
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_correlations_scope ON happy_wakey_correlation_findings
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
CREATE POLICY happy_wakey_chat_scope ON happy_wakey_chat_sessions
  USING (tenant_id = current_setting('app.tenant_id', true) AND subject_id = current_setting('app.subject_id', true));
