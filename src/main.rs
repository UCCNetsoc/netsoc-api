use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as AResult};
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    event_id: i64,
    name: String,
    date: String,
    location: String,
    public: bool,
    image_url: String
}

fn select_from_database(query: &str) -> AResult<impl Responder> {
    let conn = Connection::open("db/database.db")
        .map_err(|e| {
            // Convert the rusqlite error into an Actix Web error
            actix_web::error::ErrorInternalServerError(e)
        })?;


    let mut stmt = conn.prepare(query)
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
        })?;


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

#[get("/")]
async fn events_api() -> AResult<impl Responder> {
    return select_from_database("SELECT * FROM events;");
}

#[get("/upcoming_events")]
async fn upcoming_events_api() -> AResult<impl Responder> {
    return select_from_database("SELECT * FROM events WHERE date > datetime('now', 'localtime');");
}

#[get("/past_events")]
async fn past_events_api() -> AResult<impl Responder> {
    return select_from_database("SELECT * FROM events WHERE date < datetime('now', 'localtime');");
}


fn main() -> Result<()> {
    let _ = start_server();
    Ok(())
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(events_api)
            .service(upcoming_events_api)
            .service(past_events_api)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}