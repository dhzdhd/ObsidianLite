use std::time::Duration;

use chrono::Days;
use poise::serenity_prelude::{CacheHttp, Member, PermissionOverwrite, Permissions, Timestamp};
use tokio::time::sleep;

use crate::{Context, Error};

/// Mutes a guild member
///
/// Mutes members for a given duration.
/// If the duration is not specified, the member is muted until unmute is invoked.
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_MESSAGES",
    category = "Moderation",
    guild_only
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The member you want to mute"] mut member: Member,
    #[description = "Duration in minutes. Member muted till unmute invoked. if not specified"]
    duration: Option<u64>,
) -> Result<(), Error> {
    // member.disable_communication_until_datetime(ctx.http());

    Ok(())
}
