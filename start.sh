#!/bin/bash

docker compose down
docker compose up
docker exec basketball-database-1 ./cockroach --host=database-1:26357 init --insecure