-- Add up migration script here
CREATE TABLE event (
    event_id INTEGER PRIMARY KEY,
    guild_id INTEGER NOT NULL,
    durations TEXT NOT NULL
)
