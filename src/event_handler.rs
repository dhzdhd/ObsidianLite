use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;

use crate::{Data, Error};

pub async fn event_handler<'a>(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            println!("Bot ready!")
        }
        FullEvent::GuildScheduledEventCreate { event } => {
            event.start_time;
            &event.name;
            &event.description;
            event.id;
        }
        _ => {}
    }

    Ok(())
}
