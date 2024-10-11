HOST=$1

cockroach start \
 --insecure \
 --cache=25% \
 --max-sql-memory=25% \
 --locality="region=eu-central,zone=eu-poland-1" \
 --http-addr="$HOST:8080" \
 --listen-addr="$HOST:26357" \
 --sql-addr="$HOST:26257" \
 --join="$HOST:26357,database-2:26357,database-3:26357" \
&& cockroach init \
 --insecure \
 --host="$HOST:26357" \
&& cockroach sql \
 --insecure \
 --host="$HOST:26257" \
 --execute="CREATE DATABASE basketball; CREATE USER basketball WITH PASSWORD NULL; ALTER DATABASE basketball OWNER TO basketball; CREATE TYPE basketball.position AS ENUM ('PG', 'SG', 'SF', 'PF', 'C'); ALTER TYPE basketball.position OWNER TO basketball;"
