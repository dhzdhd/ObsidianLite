use crate::{Data, Error};
use poise::serenity_prelude::prelude::SerenityError;
use poise::serenity_prelude::{Colour, CreateEmbed, Timestamp};
use poise::{CreateReply, FrameworkError};

pub fn error_embed(message: String) -> CreateReply {
    CreateReply::default()
        .embed(
            CreateEmbed::new()
                .title("Error!")
                .description(message)
                .colour(Colour::RED)
                .timestamp(Timestamp::now()),
        )
        .ephemeral(true)
}

pub async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            if let Err(err) = ctx.send(error_embed(error.to_string())).await {
                println!("Command error: {:#?}", err);
            };
        }
        FrameworkError::EventHandler { error, .. } => {
            println!("Event error: {:#?}", error);
        }
        FrameworkError::Setup { error, .. } => {
            println!("Setup error: {:#?}", error);
        }
        other => {
            println!("An unexpected error occurred: {:#?}", other);
        }
    }
}

#[derive(Debug)]
pub enum BotError {
    FrameworkError,
    DatabaseError,
    IOError,
    SerdeError,
    APIError,
}

impl From<SerenityError> for BotError {
    fn from(_value: SerenityError) -> Self {
        Self::FrameworkError
    }
}

impl From<sqlx::Error> for BotError {
    fn from(_value: sqlx::Error) -> Self {
        Self::DatabaseError
    }
}

impl From<std::io::Error> for BotError {
    fn from(_value: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<serde_json::Error> for BotError {
    fn from(_value: serde_json::Error) -> Self {
        Self::SerdeError
    }
}

impl From<reqwest::Error> for BotError {
    fn from(_value: reqwest::Error) -> Self {
        Self::APIError
    }
}
