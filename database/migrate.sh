#!/usr/bin/env bash
set -euo pipefail

DB_URL="${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/rok_api_dev}"

echo "→ Applying migrations to ${DB_URL}"

for f in database/migrations/*.sql; do
    echo "  • $(basename "$f")"
    psql "${DB_URL}" -q -f "$f"
done

echo "✓ Migrations complete"
