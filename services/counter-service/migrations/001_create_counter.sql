-- Counter table migration
-- Creates the counter table for tenant-scoped counters.
-- This migration is idempotent (CREATE TABLE IF NOT EXISTS).

CREATE TABLE IF NOT EXISTS counter (
    tenant_id TEXT PRIMARY KEY,
    value INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
