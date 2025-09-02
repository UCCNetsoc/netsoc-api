use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as AResult};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    event_id: i64,
    name: String,
    date: String,
    location: String,
    public: bool,
    image_url: String
}


#[get("/")]
async fn api() -> AResult<impl Responder> {
    let conn = Connection::open("src/database.db")
        .map_err(|e| {
            // Convert the rusqlite error into an Actix Web error
            actix_web::error::ErrorInternalServerError(e)
        })?;


    let mut stmt = conn.prepare("SELECT * FROM events;")
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })?;



    let response = stmt.query_map([], |row| {
        Ok(Event {
            event_id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            location: row.get(3)?,
            public: row.get(4)?,
            image_url: row.get(5)?
        })
    })
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })?;;


    // Making a vector to store events in
    let mut events = Vec::new();
    for event in response {
        match event {
            Ok(e) => {
                if e.public {
                    events.push(e);
                }
            }
            Err(e) => { eprintln!("Error reading row: {}", e); }
        }
    }

    let json: String = json!(events).to_string();

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json)
    )
}

fn main() -> Result<()> {
    start_server();
    Ok(())
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}