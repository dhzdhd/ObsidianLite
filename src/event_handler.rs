use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;
use sqlx::query;

use crate::{Data, Error};

pub async fn event_handler<'a>(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let database = &data.database;

    match event {
        FullEvent::Ready { data_about_bot } => {
            println!("Bot ready!")
        }
        FullEvent::GuildScheduledEventCreate { event } => {
            let event_id = event.id.get() as i64;
            let guild_id = event.guild_id.get() as i64;
            query!("INSERT INTO event VALUES(?,?)", event_id, guild_id)
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
