./cockroach start                               \
  --insecure                                    \
  --cache=25%                                   \
  --max-sql-memory=25%                          \
  --locality=region=eu-central,zone=eu-poland-1 \
  --listen-addr="database-1:26357"              \
  --sql-addr="database-1:26257"                 \
  --join=database-1:26357,database-2:26357,database-3:26357

./cockroach init \
  --insecure     \
  --host="basketball-database-1:26357"

./cockroach sql                        \
  --insecure                           \
  --host="basketball-database-1:26257" \
  --execute="CREATE DATABASE basketball; CREATE USER basketball WITH PASSWORD NULL; ALTER DATABASE basketball OWNER TO basketball; CREATE TYPE basketball.position_enum AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');"

./cockroach node status \
  --insecure            \
  --host="basketball-database-1:26257"