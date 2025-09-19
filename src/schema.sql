CREATE TABLE events (
    event_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    date DATETIME NOT NULL,
    location TEXT NOT NULL,
    public BOOLEAN NOT NULL,
    image_url TEXT NOT NULL,
    description TEXT NOT NULL
);

SELECT * from events;

INSERT INTO events (name, date, location, public, image_url, description)
    VALUES (
        "Pizza, Games (video + board) & Chill",
        "2025-09-24 18:00:00",
        "???",
        true,
        "N/A",
        "A fun chill evening for meeting new friends, playing games, and of course eating pizza :3"
    );




INSERT INTO events (name, date, location, public, image_url)
    VALUES (
        "Demo Event",
        "2026-01-01 11:11:11",
        "WGB G.04",
        false,
        "N/A"
    );