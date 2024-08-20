# basketball

Linux/MacOS
```bash
git clone https://github.com/botprzemek/basketball ~/basketball

cd ~/basketball

docker compose down -d --volumes --remove-orphans &>/dev/null
docker compose up -d --build --force-recreate &>/dev/null
docker exec -it "basketball-database-1" ./cockroach init --insecure --host="basketball-database-1:26357" &>/dev/null
docker exec -it "basketball-database-1" ./cockroach sql --insecure --host="basketball-database-1:26257" --execute="CREATE DATABASE basketball; CREATE USER basketball WITH PASSWORD NULL; ALTER DATABASE basketball OWNER TO basketball; CREATE TYPE basketball.position_enum AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');" &>/dev/null
docker exec -it "basketball-database-1" ./cockroach node status --insecure --host="basketball-database-1:26257"
```

Windows
```bash
git clone https://github.com/botprzemek/basketball ~/basketball

cd ~/basketball

docker-compose down -v --remove-orphans > $null 2>&1
docker-compose up -d --build --force-recreate > $null 2>&1
docker exec -it basketball-database-1 ./cockroach init --insecure --host="basketball-database-1:26357" > $null 2>&1
docker exec -it basketball-database-1 ./cockroach sql --insecure --host="basketball-database-1:26257" --execute="CREATE DATABASE basketball; CREATE USER basketball WITH PASSWORD NULL; ALTER DATABASE basketball OWNER TO basketball; CREATE TYPE basketball.position_enum AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');" > $null 2>&1
docker exec -it basketball-database-1 ./cockroach node status --insecure --host="basketball-database-1:26257"
```
