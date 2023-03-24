use anyhow::Context as _;
use extensions::moderator::mute::mute;
use extensions::{fun::weather::weather, utils::help::help};
use poise::serenity_prelude::{self as serenity, GuildId};
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

mod extensions;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token: String;
    if cfg!(debug_assertions) {
        discord_token = secret_store
            .get("DEV_DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")?;
    } else {
        discord_token = secret_store
            .get("DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")?;
    }

    let dev_guild_id = secret_store
        .get("DEV_GUILD_ID")
        .context("'DEV_GUILD_ID' was not found")?
        .parse::<u64>()
        .expect("The Guild ID is not an integer!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), weather(), mute(), help()],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::all())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::from(dev_guild_id),
                )
                .await?;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
