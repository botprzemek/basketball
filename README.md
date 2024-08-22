# basketball

Linux/MacOS

Initialize

```bash
git clone https://github.com/botprzemek/basketball ~/basketball

cd ~/basketball
````

Start

```bash
docker compose down -d --volumes --remove-orphans &>/dev/null
docker compose up -d --build --force-recreate &>/dev/null
docker exec -it basketball-database-1 ./cockroach init --insecure --host="basketball-database-1:26357" &>/dev/null
docker exec -it basketball-database-1 ./cockroach sql --insecure --host="basketball-database-1:26257" --execute="CREATE DATABASE basketball; CREATE USER basketball WITH PASSWORD NULL; ALTER DATABASE basketball OWNER TO basketball; CREATE TYPE basketball.position_enum AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');" &>/dev/null
docker exec -it basketball-database-1 ./cockroach node status --insecure --host="basketball-database-1:26257"
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

# botprzemek's Basketball API

Development

npm run install

npm run dev

Building

npm run install

npm run build

npm run start

Deployment

docker compose up

> [!Caution]
> <span id="status">Project is still work in progress</span>

https://youtu.be/_1IKwnbscQU?si=U3HJeYzsupcBohvN
https://youtu.be/GmXPwRNIrAU?si=RJVPFVS1AHvJag6-
https://youtu.be/6WZ6S-qmtqY?si=eePsuPFUwczIGOVi
https://youtu.be/_gQaygjm_hg?si=WGZect2sf7qOBT8O

NGINX rate limiter

middlewares
accessList.ts
accessToken.ts
dataValidation.ts
headers.ts
webToken.ts
utils
logger.ts
paginate.ts
queryParameter.ts
rateLimiter.ts
version.ts

tests
server
api
delete
get
post
put

services
analitics
auth
data

TODO

- ADD IMAGES

1. Cache invalidation

With cache invalidation, whenever a value is updated in the primary database, each cached item with a corresponding key
is automatically deleted from the cache or caches. Although cache invalidation could perhaps be seen as a “brute force
approach,” the advantage is that it requires only one costly and often time-consuming write—to the primary database
itself—instead of two or more.

2. Write-through caching

In this case, rather than updating the primary database and removing the cache, with the write-through strategy, the
application updates the cache, and then the cache updates the primary database synchronously. In other words, instead of
relying on the primary database to initiate any updating, the cache is in charge of maintaining its own consistency and
delivering word of any changes it makes back to the primary database.

3. Write-behind caching

Unfortunately, there are times when two writes can actually make a wrong. One of the drawbacks of the write-through
cache strategy is that updating both the cache and the primary database requires two time-consuming, processor-taxing
changes, first to the cache and then to the primary database.

Another strategy, known as write-behind, avoids this problem by initially updating only the cache and then updating the
primary database later. Of course, the primary database will also need to be updated, and the sooner the better, but in
this case the user doesn’t have to pay the “cost” of the two writes. The second write to the primary database occurs
asynchronously and behind the scenes (hence the name, write-behind) at a time when it is less likely to impair
performance.

Auth

Hypermedia HATEOAS

GZIP Compression

GET, POST, PUT, DELETE

NODEJS, TYPESCRIPT, HTTP, REDIS, COCKROACH, DOCKER, MULTI-INSTANCE

statistics

parameters

database schema

api token

rate limit

ip exclude

admin ui

tests

swagger
openapi

mocking

deployment

## <span id="overview">Project Overview :memo:</span>

Stack of whole project including
[website project](https://github.com/botprzemek/basketball-website) - **CENN (CockroachDB, Express,
Nuxt, NodeJS)**

This documentation provides an overview of the REST API project developed for the Knury Knurów
basketball team. The API is built around JavaScript (TypeScript :milky_way:) with Express :coffee:
framework (Node.js). It integrates technologies such as CockroachDB :cockroach: and Socket.IO
:satellite:, Node-Mailer :mailbox: and Node-Cache :eight_pointed_black_star:

## Navigation :busstop:

1. [Status](#status)
2. [Project Overview](#overview)
3. [Quick Start](#setup)
4. [Features](#features)
5. [Usage](#usage)
6. [Technologies used](#technologies)
7. [Endpoints](#endpoints)
8. [Database Models](#database)
9. [Contributors](#contributors)
10. [Author](#author)
11. [License](#license)

## <span id="setup">Quick Start :rocket:</span>

Testing API instance running on [this link](https://api.testing.knuryknurow.pl/), please refer to
API's endpoints

1. Install Node

2. Clone repository git clone https://github.com/botprzemek/basketball-api.git

cd basketball-api

npm install

cp .env.example .env

setup env file

```dotenv
PORT=3000
ADDRESSES=["http://localhost:3001"]

SECRET=generated_secret

COCKROACH_HOST=your.database.domain.com
COCKROACH_PORT=26257
COCKROACH_NAME=api
COCKROACH_USER=your_user
COCKROACH_PASSWORD=your_database_password

MAIL_URL=your.mail.domain.com
MAIL_PORT=587
MAIL_USER=your_user
MAIL_PASSWORD=your_mail_password

TOKEN_KEY=generated_token
```

npm run dev

To run the project in development mode (Nodemon - Restarting after changes), execute the following
command:

```shell
npm run dev
```

To run the seeding script to fill your database, execute the following command:

```shell
npm run seedling
```

To run the project in production mode (PM2 - daemonize app and logging), execute the following
command:

```shell
npm run server
```

## <span id="usage">Usage :tada:</span>

<!-- TODO API AND SOCKET -->

## <span id="technologies">Technologies Used :bulb:</span>

1. [**JavaScript**](https://developer.mozilla.org/en-US/docs/Web/JavaScript): Programming language
   used for the project's core functionality. :toolbox:
2. [**TypeScript**](https://www.typescriptlang.org/docs): Superset of mentioned above JavaScript,
   used for adding static typing and enhancing code maintainability. :link:
3. [**Node**](https://nodejs.org/en/docs) An asynchronous event-driven JavaScript runtime, designed
   to build scalable network applications. :crystal_ball:
4. [**CockroachDB**](https://www.cockroachlabs.com/docs): Database engine used for storing and
   retrieving team-related data. :file_folder:
5. [**Express**](https://expressjs.com/en/4x/api): Web application framework for building the API's
   routes and handling requests. :printer:
6. [**Node-Cache**](https://github.com/node-cache/node-cache): Caching mechanisms implemented using
   Node-Cache package to optimize API performance. :package:
7. [**Node-Mailer**](https://nodemailer.com/usage/): Allows easy as cake email sending. :mailbox:
8. [**Socket.IO**](https://socket.io/docs/v4/server-api): Caching mechanisms implemented using
   Node-Cache package to optimize API performance. :package:

## <span id="database">Database Models :abacus:</span>

This section provides an overview of the structure, including basketball arenas, cities, players,
teams, statistics, matches, and rosters, along with their respective relationships inside the
CockroachDB database.

### Player

- **id** (INT8): Primary key, unique identifier for the player.
- **team_id** (INT8): Foreign key referencing the team table (not null, references team.id, on
  delete cascade).
- **name** (VARCHAR): First name of the player (not null).
- **lastname** (VARCHAR): Last name of the player (not null).
- **number** (INT8): Player number (not null, unique within a team).
- **height** (INT8): Height of the player (not null).
- **position** (position_enum): Position of the player (not null).
- **birthday** (DATE): Birthday of the player (not null).
- **starter** (BOOLEAN): Indicates if the player is a starter (default false).

## <span id="endpoints">Endpoints :satellite:</span>

<!-- TODO ENDPOINTS -->

Contributors

This project follows the all-contributors specification and is brought to you by these awesome
contributors.

author

Github @botprzemek Discord botprzemek Email info@botprzemek.pl

license

This project is licensed under the terms of the Apache License 2.0

http2 protobuf
