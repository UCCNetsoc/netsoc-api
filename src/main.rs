use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Event {
    event_id: i64,
    name: String,
    date: String,
    location: String,
    public: bool,
    image_url: String
}

fn main() -> Result<()> {

    let conn = Connection::open("src/database.db")?;

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
        match event {
            Ok(e) => {
                println!("Found event with details:");
                println!("Name: {:} \nLocation: {:} \nDate/Time: {:}", e.name, e.location, e.date);
                
            }
            Err(e) => eprintln!("Error reading row: {}", e),
        }
    }
    Ok(())
}