use poise::builtins;

use crate::{Context, Error};

/// Help command
#[poise::command(slash_command, category = "Utilities", hide_in_help)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = builtins::HelpConfiguration {
        ..Default::default()
    };
    builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
