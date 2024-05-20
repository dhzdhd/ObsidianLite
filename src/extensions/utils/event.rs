use poise::builtins;

use crate::{Context, Error};

/// Event command
///
/// Create customized events from already set events
#[poise::command(slash_command, category = "Utilities", hide_in_help)]
pub async fn event(
    ctx: Context<'_>,
    #[description = "Duration in hours"] duration: Option<i64>,
) -> Result<(), Error> {
    Ok(())
}
