#!/bin/bash

host="basketball-database-1"
user="basketball"
sql="CREATE DATABASE ${user}; CREATE USER ${user} WITH PASSWORD NULL; ALTER DATABASE ${user} OWNER TO ${user}; CREATE TYPE ${user}.position_enum AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');"

docker compose down -d --volumes --remove-orphans &>/dev/null
docker compose up -d --build --force-recreate &>/dev/null
docker exec -it "${host}" ./cockroach init --host="${host}:26357" --insecure &>/dev/null
docker exec -it "${host}" ./cockroach sql --host="${host}:26257" --insecure --execute="${sql}" &>/dev/null
docker exec -it "${host}" ./cockroach node status --host="${host}:26257" --insecure