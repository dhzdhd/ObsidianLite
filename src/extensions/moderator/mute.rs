use std::time::Duration;

use poise::serenity_prelude::{CacheHttp, Member, Permissions};
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
    if let Some(guild) = ctx.guild() {
        let role = match guild.role_by_name("muted") {
            Some(r) => r.clone(),
            None => {
                guild
                    .create_role(ctx.http(), |r| {
                        r.name("muted")
                            .position(guild.roles.values().len() as u8)
                            .hoist(true)
                    })
                    .await?
            }
        };

        member.add_role(ctx.http(), role.id).await?;

        ctx.say("Muted user").await?;

        if let Some(duration) = duration {
            sleep(Duration::from_millis(duration * 60 * 1000)).await;

            member.remove_role(ctx.http(), role.id).await?;

            ctx.say("Unmuted").await?;
        }
    }

    Ok(())
}
