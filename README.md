# UCC NetSoc API

## Events / Calender API
This is the API that any of NetSoc's tools can use to figure out what events are happening in the NetSoc calender. For example; our Website or Discord Bots.

## Building
> Note: Listens on port 8080

cargo build --release`
- `docker compose up --build`

## Backing up the Database
run `docker cp {INSERT CONTAINER ID HERE}:/app/db/database.db database-{INSERT CURRENT DATE HERE}.db`

## Running
- Run `docker compose up`

## Production
docker pull from `yakowa/netsoc-api`