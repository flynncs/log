CREATE TYPE log_level AS ENUM(
  'debug',
  'info',
  'warn',
  'error'
);

ALTER TABLE log_entries
  ALTER COLUMN level TYPE log_level
  USING level::log_level;

