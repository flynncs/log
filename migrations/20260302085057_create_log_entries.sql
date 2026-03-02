CREATE TABLE log_entries(
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  timestamp timestamptz NOT NULL DEFAULT now(),
  level text NOT NULL,
  service text NOT NULL,
  message text NOT NULL,
  attributes jsonb NOT NULL DEFAULT '{}',
  trace_id text,
  span_id text
);

CREATE INDEX idx_log_entries_timestamp ON log_entries(timestamp DESC);

CREATE INDEX idx_log_entries_level ON log_entries(level);

CREATE INDEX idx_log_entries_service ON log_entries(service);

