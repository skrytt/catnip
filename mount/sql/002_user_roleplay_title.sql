BEGIN;

ALTER TABLE Users ADD COLUMN Title TEXT;

PRAGMA user_version=2;

COMMIT;
