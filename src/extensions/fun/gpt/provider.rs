use crate::{extensions::fun::gpt::ws_helper::open_tls_stream, Context, Error};
use chrono::Utc;
use poise::futures_util::SinkExt;
use poise::serenity_prelude::Color;
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, handshake::client::Response};
use url::Url;

// use futures_util::{SinkExt, StreamExt};
// use log::*;
use poise::futures_util::StreamExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};
// use tokio_tungstenite::Message;
use tokio_tungstenite::{accept_async, client_async, WebSocketStream};

// type WebSocketStream =
//     tokio_tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>>;

#[poise::command(slash_command)]
pub async fn gpt(
    ctx: Context<'_>,
    // #[description = "Enter a provider"] provider: String,
) -> Result<(), Error> {
    let ws_url = Url::parse("wss://api.myshell.ai/ws/?EIO=4&transport=websocket").unwrap();
    let tls = open_tls_stream(&ws_url).await;
    let (mut ws, _) = tokio_tungstenite::client_async(ws_url.into_client_request().unwrap(), tls)
        .await
        .unwrap();

    // ws.send(Message::Text("hi".to_string()));
    let message = ws.next().await.unwrap();
    let s = format!("{:?}", message);

    // let response = reqwest::get(format!("https://goweather.herokuapp.com/weather/{city}"))
    //     .await
    //     .map_err(|_| "API endpoint unreachable!")?
    //     .json::<Weather>()
    //     .await
    //     .map_err(|_| "Weather for the given city does not exist!")?;

    ctx.send(|b| {
        b.embed(|e| {
            let user = ctx.author();

            e.title("GPT")
                .description(format!("{}", s))
                .footer(|f| {
                    f.text(&user.name)
                        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()))
                })
                .colour(Color::BLURPLE)
                .thumbnail(
                    "https://www.freeiconspng.com/thumbs/weather-icon-png/weather-icon-png-2.png",
                )
                .timestamp(Utc::now())
        })
    })
    .await?;

    Ok(())
}
