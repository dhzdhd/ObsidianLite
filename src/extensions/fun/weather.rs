use chrono::Utc;
use poise::serenity_prelude::Color;
use serde::Deserialize;

use crate::{Context, Error};

#[derive(Deserialize)]
struct Forecast {
    day: String,
    temperature: String,
    wind: String,
}

#[derive(Deserialize)]
struct Weather {
    temperature: String,
    wind: String,
    description: String,
    forecast: Vec<Forecast>,
}

#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Enter a city"] city: String,
) -> Result<(), Error> {
    let response = reqwest::get(format!("https://goweather.herokuapp.com/weather/{}", city))
        .await
        .map_err(|_| "API endpoint unreachable!")?
        .json::<Weather>()
        .await
        .map_err(|_| "Weather for the given city does not exist!")?;

    ctx.send(|b| {
        b.embed(|e| {
            let user = ctx.author();

            e.title(format!(
                "It is currently {} in {}",
                response.description, city
            ))
            .description(format!(
                "Temperature {}\n Wind{}",
                response.temperature, response.wind
            ))
            .footer(|f| f.text(&user.name).icon_url(user.avatar_url().unwrap_or(user.default_avatar_url())))
            .colour(Color::BLURPLE)
            .thumbnail("https://camo.githubusercontent.com/1aab89eda0e0e99d9aa2e7318d8161c370bfb6d75c2945a60f3b0cd1eb8a8c3c/68747470733a2f2f626d63646e2e6e6c2f6173736574732f776561746865722d69636f6e732f76332e302f66696c6c2f7376672f706172746c792d636c6f7564792d6461792e737667")
            .timestamp(Utc::now())
            .fields(response.forecast.iter().map(|el| {
                (
                    format!("Day {}", el.day),
                    format!("Temperature {}\nWind {}", el.temperature, el.wind),
                    false,
                )
            }))
        })
    })
    .await?;

    Ok(())
}
