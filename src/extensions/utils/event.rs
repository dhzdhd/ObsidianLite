use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude::{GuildId, Http, ScheduledEventId};
use sqlx::query;

use crate::{Context, Data, Error};

pub async fn event_periodic_task(ctx: Arc<Http>, data: Data) -> Result<(), Error> {
    loop {
        let database = &data.database;

        let result = query!("SELECT * FROM event").fetch_all(database).await?;
        for record in result.iter() {
            let guild_id = GuildId::new(record.guild_id as u64);
            let event_id = ScheduledEventId::new(record.event_id as u64);

            // let guild = ctx.get_guild(guild_id).await?;
            let event = ctx.get_scheduled_event(guild_id, event_id, true).await?;

            println!("{:#?}", event.start_time);
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

/// Event command
///
/// Create customized events from already set events
#[poise::command(slash_command, category = "Utilities", hide_in_help)]
pub async fn event(
    ctx: Context<'_>,
    #[description = "Duration in hours"] duration: Option<i64>,
) -> Result<(), Error> {
    // ctx.guild()
    //     .unwrap()
    //     .create_scheduled_event(
    //         ctx.http(),
    //         CreateScheduledEvent::new(guild::ScheduledEventType::External, "hi", Timestamp::now()),
    //     )
    //     .await;
    ctx.say("text").await?;

    Ok(())
}
