#![feature(type_alias_impl_trait)]
mod handlers;
mod mq;
mod types;

use std::thread::sleep;
use std::time::Duration;

use lapin::options::BasicAckOptions;
use mq::{ensure_exchange, ensure_queue, get_connection};
use rand::random;
use tokio;
use types::{BoardgameSite, Message};

async fn handle_message(site: &BoardgameSite, message: &Message) {
    let _result = match site {
        BoardgameSite::Spelonk => handlers::spelonk::handle(message).await,
        BoardgameSite::ThePlayground => todo!(),
    };
}

async fn listen_for_site(site: &BoardgameSite) -> Result<(), lapin::Error> {
    let conn = get_connection().await?;
    let channel = conn.create_channel().await?;
    ensure_exchange(&channel).await?;
    ensure_queue(&channel, &site).await?;

    println!("Listening for site {:?}", site);

    loop {
        let msg = channel
            .basic_get(
                &site.to_string(),
                lapin::options::BasicGetOptions::default(),
            )
            .await?;

        if let Some(msg) = msg {
            let message: Message =
                serde_json::from_slice(&msg.data).expect("Deserialization failed");

            handle_message(&site, &message).await;

            channel
                .basic_ack(msg.delivery_tag, BasicAckOptions::default())
                .await?;
        } else {
            sleep(Duration::from_secs(1));
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), lapin::Error> {
    let site = if random::<f32>() > 0.5 {
        BoardgameSite::Spelonk
    } else {
        BoardgameSite::ThePlayground
    };

    listen_for_site(&site).await
}
