# App Flow
sqlite database stores all event information
rust script queries db based on arguments givin by requester
rust script converts response to JSON
api.netsoc.co returns a JSON response
Website/Discord Bot/etc ---> api.netsoc.co (passing arguments for specifics (e.g. past or present events) via get/post)

## Technologies
- SQLite
- Rust
- Docker
- NGINX