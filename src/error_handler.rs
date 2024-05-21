use crate::Data;
use poise::serenity_prelude::prelude::SerenityError;
use poise::FrameworkError;

#[derive(Debug)]
pub enum BotError {
    FrameworkError,
    DatabaseError,
    IOError,
    SerdeError,
    APIError,
}

impl From<SerenityError> for BotError {
    fn from(value: SerenityError) -> Self {
        Self::FrameworkError
    }
}

impl From<sqlx::Error> for BotError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError
    }
}

impl From<std::io::Error> for BotError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<serde_json::Error> for BotError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError
    }
}

impl From<reqwest::Error> for BotError {
    fn from(value: reqwest::Error) -> Self {
        Self::APIError
    }
}

// pub async fn on_error<'a>(error: FrameworkError<'a, Data, BotError>) {
//     match error {
//         FrameworkError::Command { error, ctx } => {
//             // Handle command-specific errors
//             let _ = ctx
//                 .say(format!(
//                     "An error occurred while executing the command: {:#?}",
//                     error
//                 ))
//                 .await;
//         }
//         FrameworkError::Setup {
//             error,
//             framework,
//             data_about_bot,
//             ctx,
//         } => {
//             // Handle setup errors
//             println!("Setup error: {:#?}", error);
//         }
//         // FrameworkError::Dispatch { error } => {
//         //     // Handle dispatch errors
//         //     println!("Dispatch error: {}", error);
//         // }
//         other => {
//             // Handle other types of errors
//             println!("An unexpected error occurred: {:#?}", other);
//         }
//     }
// }
