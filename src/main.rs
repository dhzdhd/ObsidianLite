use dotenvy::dotenv;
// use error_handler::{on_error, BotError};
use event_handler::event_handler;
use extensions::moderator::mute::mute;
use extensions::utils::event::event_periodic_task;
use extensions::{fun::weather::weather, utils::event::event, utils::help::help};
use poise::serenity_prelude::{self as serenity, GuildId};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

mod error_handler;
mod event_handler;
mod extensions;

#[derive(Clone, Debug)]
pub struct Data {
    pub database: SqlitePool,
}

impl Data {
    fn new(database: SqlitePool) -> Self {
        Self { database }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
// pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    match dotenv() {
        Ok(_) => println!(".env file loaded"),
        Err(_) => println!(".env file failed to load"),
    }

    let discord_token = if cfg!(debug_assertions) {
        std::env::var("DEV_DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found")
    } else {
        std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found")
    };

    let dev_guild_id = std::env::var("DEV_GUILD_ID")
        .expect("'DEV_GUILD_ID' was not found")
        .parse::<u64>()
        .expect("The Guild ID is not an integer!");

    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Could not connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let data = Data::new(database);
    let data_clone = data.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), weather(), mute(), event(), help()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            // on_error: |error| Box::pin(on_error(error)),
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
                Ok(data.clone())
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::all())
        .framework(framework)
        .await?;

    tokio::join!(
        event_periodic_task(client.http.clone(), data_clone),
        client.start(),
    )
    .0?;

    Ok(())
}
