use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;
use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::{Data, Error};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventDurationKind {
    StartOfDay, // 7 AM
    Hour(u64),
    Minute(u64),
}

pub async fn event_handler<'a>(
    _ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let database = &data.database;

    match event {
        FullEvent::Ready { data_about_bot: _ } => {
            println!("Bot ready!")
        }
        FullEvent::GuildScheduledEventCreate { event } => {
            let event_id = event.id.get() as i64;
            let guild_id = event.guild_id.get() as i64;

            // By default - start of day & 1 hour before event
            let durations = vec![EventDurationKind::StartOfDay, EventDurationKind::Hour(1)];
            let durations_string = serde_json::to_string::<Vec<EventDurationKind>>(&durations)?;

            query!(
                "INSERT INTO event VALUES(?,?,?)",
                event_id,
                guild_id,
                durations_string
            )
            .execute(database)
            .await?;
        }
        FullEvent::GuildScheduledEventDelete { event } => {
            let event_id = event.id.get() as i64;
            query!("DELETE FROM event WHERE event_id=?", event_id)
                .execute(database)
                .await?;
        }
        _ => {}
    }

    Ok(())
}
