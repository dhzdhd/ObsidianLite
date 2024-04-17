// use anyhow::Context as _;
use dotenvy::dotenv;
use extensions::moderator::mute::mute;
use extensions::{fun::weather::weather, utils::help::help};
use poise::serenity_prelude::{self as serenity, GuildId};
// use shuttle_poise::ShuttlePoise;
// use shuttle_secrets::SecretStore;

mod extensions;

pub struct Data {} // User data
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let discord_token = if cfg!(debug_assertions) {
        std::env::var("DEV_DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found")
    } else {
        std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found")
    };

    let dev_guild_id = std::env::var("DEV_GUILD_ID")
        .expect("'DEV_GUILD_ID' was not found")
        .parse::<u64>()
        .expect("The Guild ID is not an integer!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), weather(), mute(), help()],
            ..Default::default()
        })
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
        .build();

    let client = serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::all())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
