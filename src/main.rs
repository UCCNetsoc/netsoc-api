use rusqlite::{params, Connection, Result};

struct Event {
    event_id: i64,
    name: String,
    date: String,
    location: String,
    public: bool,
    image_url: String
}

fn main() -> Result<()> {

    let conn = Connection::open_in_memory()?;

    let mut stmt = conn.prepare("SELECT * FROM events;")?;
    let response = stmt.query_map([], |row| {
        Ok(Event {
            event_id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            location: row.get(3)?,
            public: row.get(4)?,
            image_url: row.get(5)?
        })
    })?;

    for event in response {
        println!("Found event {:?}", event?);
    }
    Ok(())
}