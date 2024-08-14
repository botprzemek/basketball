#!/bin/bash

docker compose down -d --remove-orphans
docker compose up -d
docker exec basketball-database-1 ./cockroach --host=basketball-database-1:26357 init --insecure
