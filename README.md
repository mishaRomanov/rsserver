# A simple log collector service written in Rust. Learning purposes.
## Usage
- Clone this repository:
```bash
git clone git@github.com:mishaRomanov/rsserver.git
```
- Get started:
```bash
cd rsserver
make help
```
```
run      -> run service
run_detached     -> run in detached mode (with -d flag)
kill     -> stop all containers
rerun    -> self-explainatory. basically calls kill and then run
logs db  -> look at db's logs in --follow mode⏎
```
- Add `JWT_SECRET` environment variable to `.env` file. Otherwise, default value will be used.

## Available endpoints
- GET / -> basic root handler
- POST /auth -> generate jwt token with such payload:
```json
{
    "name": "John Doe",
    "email": "user@example.com",
}
```
- GET /logs -> fetch all existsing logs
- POST /log -> write a new log to database with such payload:
```json
{
  "time":"Mon, 23 Jun 2025 13:28:37 +0200",
  "level":"INFO",
  "message":"test_message"
}
```
