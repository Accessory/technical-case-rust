#!/bin/sh
set -e
echo "Start migration"
/bin/sqlx database create --database-url "$DATABASE_URL"
/bin/sqlx migrate run --database-url "$DATABASE_URL" --source /usr/migrations
echo "Start Backend"
/bin/technical-case-rust