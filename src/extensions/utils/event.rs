use std::sync::Arc;
use std::time::Duration;

use poise::{
    serenity_prelude::{GuildId, Http, ScheduledEventId},
    ChoiceParameter, CreateReply,
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

            let event = ctx
                .get_scheduled_event(guild_id, event_id, true)
                .await
                .map_err(|_| "Event not found")?;

            // println!("{:?}\n{:?}\n", event.start_time, durations);
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

#[derive(Debug, ChoiceParameter, PartialEq, Eq)]
pub enum EventChoiceParameter {
    StartOfDay,
    Hour,
    Minute,
}

#[poise::command(
    slash_command,
    category = "Utilities",
    subcommands("add", "delete", "show"),
    subcommand_required
)]
pub async fn event(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Event command
///
/// Create customized events from already set events
#[poise::command(slash_command, category = "Utilities")]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Event ID"] event_id: u128,
    #[description = "Kind"] kind: EventChoiceParameter,
    #[description = "Duration"] duration: Option<u64>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    // Except for StartOfDay, all other fields need duration
    if duration.is_none() && kind != EventChoiceParameter::StartOfDay {
        return Err("All choices other than StartOfDay need the duration argument".into());
    }

    let event = ctx
        .http()
        .get_scheduled_event(guild_id, ScheduledEventId::new(event_id as u64), true)
        .await?;
    let event_id = event.id.get() as i64;

    // Map choice parameter - which is discord specific - to a more suitable enum
    let event_duration_kind = match kind {
        EventChoiceParameter::StartOfDay => EventDurationKind::StartOfDay,
        EventChoiceParameter::Hour => EventDurationKind::Hour(duration.unwrap()),
        EventChoiceParameter::Minute => EventDurationKind::Minute(duration.unwrap()),
    };

    // Update durations for the particular event
    let old_durations_str = query!("SELECT durations FROM event WHERE event_id=?", event_id)
        .fetch_one(database)
        .await
        .map_err(|_| "Failed to fetch data from database")?
        .durations;
    let mut old_durations: Vec<EventDurationKind> =
        serde_json::from_str(&old_durations_str).unwrap();

    // Do not allow duplicate entries
    if old_durations.contains(&event_duration_kind) {
        return Err("Same event duration already exists".into());
    }

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

#[poise::command(slash_command, category = "Utilities")]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Event ID"] event_id: u128,
    #[description = "Kind"] kind: EventChoiceParameter,
    #[description = "Duration"] duration: Option<u64>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    // Except for StartOfDay, all other fields need duration
    if duration.is_none() && kind != EventChoiceParameter::StartOfDay {
        return Err("All choices other than StartOfDay need the duration argument".into());
    }

    let event = ctx
        .http()
        .get_scheduled_event(guild_id, ScheduledEventId::new(event_id as u64), true)
        .await?;
    let event_id = event.id.get() as i64;

    // Map choice parameter - which is discord specific - to a more suitable enum
    let event_duration_kind = match kind {
        EventChoiceParameter::StartOfDay => EventDurationKind::StartOfDay,
        EventChoiceParameter::Hour => EventDurationKind::Hour(duration.unwrap()),
        EventChoiceParameter::Minute => EventDurationKind::Minute(duration.unwrap()),
    };

    // Update durations for the particular event
    let old_durations_str = query!("SELECT durations FROM event WHERE event_id=?", event_id)
        .fetch_one(database)
        .await
        .map_err(|_| "Failed to fetch data from database")?
        .durations;
    let old_durations: Vec<EventDurationKind> = serde_json::from_str(&old_durations_str).unwrap();

    // Exit if entry does not exist
    if !old_durations.contains(&event_duration_kind) {
        return Err("Passed event duration does not exist".into());
    }

    let durations = old_durations
        .into_iter()
        .filter(|e| e != &event_duration_kind)
        .collect();
    let durations_str = serde_json::to_string::<Vec<EventDurationKind>>(&durations)?;

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

#[poise::command(slash_command, category = "Utilities")]
pub async fn show(
    ctx: Context<'_>,
    #[description = "Event ID"] event_id: u128,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    let event = ctx
        .http()
        .get_scheduled_event(guild_id, ScheduledEventId::new(event_id as u64), true)
        .await?;
    let event_id = event.id.get() as i64;

    let durations_str = query!("SELECT durations FROM event WHERE event_id=?", event_id)
        .fetch_one(database)
        .await
        .map_err(|_| "Failed to fetch data from database")?
        .durations;
    // let durations: Vec<EventDurationKind> = serde_json::from_str(&durations_str).unwrap();

    ctx.send(
        CreateReply::default()
            .content(durations_str)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
