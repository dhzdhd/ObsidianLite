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

    ctx.say(response.description).await?;
    Ok(())
}
