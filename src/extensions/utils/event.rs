use std::sync::Arc;
use std::time::Duration;

use poise::{
    serenity_prelude::{GuildId, Http, ScheduledEventId},
    ChoiceParameter, CreateReply, ReplyHandle,
};
use sqlx::query;

use crate::{event_handler::EventDurationKind, Context, Data, Error};

pub async fn event_periodic_task(ctx: Arc<Http>, data: Data) -> Result<(), Error> {
    loop {
        let database = &data.database;

        let result = query!("SELECT * FROM event").fetch_all(database).await?;
        for record in result.iter() {
            let guild_id = GuildId::new(record.guild_id as u64);
            let event_id = ScheduledEventId::new(record.event_id as u64);
            let durations = serde_json::from_str::<Vec<EventDurationKind>>(&record.durations)?;

            // let guild = ctx.get_guild(guild_id).await?;
            let event = ctx
                .get_scheduled_event(guild_id, event_id, true)
                .await
                .map_err(|_| "Hell nah")?;

            println!("{:#?}\n{:#?}\n", event.start_time, durations);
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

#[derive(Debug, ChoiceParameter)]
pub enum EventChoiceParameter {
    StartOfDay,
    Hour,
    Minute,
}

/// Event command
///
/// Create customized events from already set events
#[poise::command(slash_command, category = "Utilities", hide_in_help)]
pub async fn event(
    ctx: Context<'_>,
    #[description = "Event ID"] event_id: u128,
    #[description = "Kind"] kind: EventChoiceParameter,
    #[description = "Duration"] duration: Option<u64>,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();
    let event = ctx
        .http()
        .get_scheduled_event(guild_id, ScheduledEventId::new(event_id as u64), true)
        .await?;
    let event_id = event.id.get() as i64;

    let event_duration_kind = match kind {
        EventChoiceParameter::StartOfDay => EventDurationKind::StartOfDay,
        EventChoiceParameter::Hour => EventDurationKind::Hour(duration.ok_or("err")?),
        EventChoiceParameter::Minute => EventDurationKind::Minute(duration.ok_or("err")?),
    };

    let old_durations_str = query!("SELECT durations FROM event WHERE event_id=?", event_id)
        .fetch_one(database)
        .await?
        .durations;
    let mut old_durations: Vec<EventDurationKind> =
        serde_json::from_str(&old_durations_str).unwrap();
    old_durations.push(event_duration_kind);
    let durations_str = serde_json::to_string::<Vec<EventDurationKind>>(&old_durations)?;

    println!("{:#?}", durations_str);

    query!(
        "UPDATE event SET durations=? WHERE event_id=?",
        durations_str,
        event_id
    )
    .execute(database)
    .await?;

    ctx.send(
        CreateReply::default()
            .content("Event updated!")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
