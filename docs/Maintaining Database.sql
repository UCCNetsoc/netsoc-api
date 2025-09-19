-- === Maintaining Database ===

INSERT INTO events (name, date, location, public, image_url, description)
    VALUES (
        "DevCon",
        "2025-09-28 10:00:00",
        "The Hub",
        true,
        "N/A",
        "DevCon is a Cork-based Tech Conference about Bridging the Gap between Students and the Tech Industry that's held annually in UCC. Showcasing the exciting and innovative technology developments from all across Ireland! See devcon.ie"
    );


UPDATE events SET location = "Boole 3" WHERE event_id = 1;