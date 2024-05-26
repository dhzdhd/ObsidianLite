use chrono::{DateTime, Duration, Local, NaiveTime};
use poise::{
    serenity_prelude::{
        colours::roles::BLUE, CacheHttp, CreateEmbed, CreateMessage, GuildId, Http,
        ScheduledEventId, Timestamp,
    },
    ChoiceParameter, CreateReply,
};
use sqlx::query;
use std::sync::Arc;

use crate::{event_handler::EventDurationKind, Context, Data, Error};

pub async fn event_periodic_task(ctx: Arc<Http>, data: Data) -> Result<(), Error> {
    loop {
        let database = &data.database;

        let result = query!("SELECT * FROM event").fetch_all(database).await?;
        for record in result.iter() {
            let guild_id = GuildId::new(record.guild_id as u64);
            let event_id = ScheduledEventId::new(record.event_id as u64);
            let event_id_i64 = event_id.get() as i64;
            let durations = serde_json::from_str::<Vec<EventDurationKind>>(&record.durations)?;
            let mut used_durations = Vec::new();

            let event = ctx
                .get_scheduled_event(guild_id, event_id, true)
                .await
                .map_err(|_| "Event not found")?;

            let members = ctx
                .get_scheduled_event_users(guild_id, event_id, None, None, Some(true))
                .await?;

            // TODO: Move to event add command
            // TODO: Also make sure start of day is ahead of the time chosen
            // let start_time = event.start_time.time();
            // let start_day = event.start_time.date_naive();
            // let seven_am = NaiveTime::from_hms_opt(7, 0, 0).unwrap();

            // if !(start_time < seven_am) {
            //     let start_of_day = start_day.and_time(seven_am);
            //     println!("{start_time:?}  {seven_am:?}");
            // }

            // Delete event if start_time has been reached
            if Timestamp::now() > event.start_time {
                query!("DELETE FROM event WHERE event_id=?", event_id_i64)
                    .execute(database)
                    .await?;

                continue;
            }

            // Create event message embed
            let event_embed = CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title(format!("Event | {}", event.name))
                    .description(event.description.unwrap_or("No description".to_owned()))
                    .fields([(
                        "Scheduled at",
                        // Gets local time instead of UTC
                        DateTime::<Local>::from_naive_utc_and_offset(
                            event.start_time.naive_utc(),
                            Local::now().offset().clone(),
                        )
                        .format("%H:%M %d/%m/%Y")
                        .to_string(),
                        false,
                    )])
                    .timestamp(Timestamp::now())
                    .color(BLUE),
            );

            // Do rest of the stuff
            let start_day = event.start_time.date_naive();
            let seven_am = NaiveTime::from_hms_opt(7, 0, 0).unwrap();

            let start_of_day_datetime = start_day.and_time(seven_am);
            let start_datetime = event.start_time.naive_utc();

            for duration in durations.clone() {
                match duration {
                    EventDurationKind::StartOfDay => {
                        // Get current time in local time (UTC + offset)
                        let local_naive_datetime = DateTime::<Local>::from_naive_utc_and_offset(
                            Timestamp::now().naive_local(),
                            Local::now().offset().clone(),
                        )
                        .naive_local();

                        if local_naive_datetime > start_of_day_datetime {
                            for member in members.iter() {
                                member
                                    .user
                                    .direct_message(ctx.http(), event_embed.clone())
                                    .await?;
                            }

                            used_durations.push(duration);
                        }
                    }
                    EventDurationKind::Hour(time) => {
                        let hour_datetime = start_datetime
                            .checked_sub_signed(Duration::hours(time as i64))
                            .unwrap();

                        if Timestamp::now().naive_utc() > hour_datetime {
                            for member in members.iter() {
                                member
                                    .user
                                    .direct_message(ctx.http(), event_embed.clone())
                                    .await?;
                            }

                            used_durations.push(duration);
                        }
                    }
                    EventDurationKind::Minute(time) => {
                        let minute_datetime = start_datetime
                            .checked_sub_signed(Duration::minutes(time as i64))
                            .unwrap();

                        if Timestamp::now().naive_utc() > minute_datetime {
                            for member in members.iter() {
                                member
                                    .user
                                    .direct_message(ctx.http(), event_embed.clone())
                                    .await?;
                            }

                            used_durations.push(duration);
                        }
                    }
                }
            }

            // Make new_durations vec which contains all durations except ones
            // that are present in used_durations (already finished)
            let new_durations = durations
                .into_iter()
                .filter(|e| !used_durations.contains(e))
                .collect::<Vec<EventDurationKind>>();
            let new_durations_str =
                serde_json::to_string::<Vec<EventDurationKind>>(&new_durations)?;

            // Update only if atleast 1 duration has been finished
            if !used_durations.is_empty() {
                query!(
                    "UPDATE event SET durations=? WHERE event_id=?",
                    new_durations_str,
                    event_id_i64
                )
                .execute(database)
                .await?;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
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
    let old_durations_str = query!(
        "SELECT durations FROM event WHERE event_id=? LIMIT 1",
        event_id
    )
    .fetch_one(database)
    .await
    .map_err(|_| "Failed to fetch data from database")? // TODO: Improve error for non existent event
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
