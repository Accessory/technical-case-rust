#!/bin/sh
echo "Start migration"
sqlx database create --database-url "$DATABASE_URL"
sqlx migrate run --database-url "$DATABASE_URL"
echo "Start Backend"
/opt/backend/technical-case-rust