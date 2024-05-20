use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude::{CacheHttp, Http};

use crate::{Context, Data, Error};

pub async fn event_periodic_task(ctx: Arc<Http>, data: Data) -> Result<(), Error> {
    loop {
        // let guild_id = ctx.http().get

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
