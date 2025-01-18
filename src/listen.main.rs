#![feature(type_alias_impl_trait)]
mod mq;
mod scrapers;
mod types;
mod util;

use std::thread::sleep;
use std::time::Duration;

use lapin::options::BasicAckOptions;
use mq::{ensure_exchange, ensure_queue, get_connection};
use rand::random;
use scrapers::spelonk::SpelonkScraper;
use scrapers::the_playground::ThePlaygroundScraper;
use scrapers::Scraper;
use tokio;
use types::{BoardgameSite, Message};
use util::RateLimitedClient;

fn get_scraper(website: &BoardgameSite) -> Box<dyn Scraper + Send + Sync> {
    match website {
        BoardgameSite::Spelonk => Box::new(SpelonkScraper {
            client: RateLimitedClient::new(Duration::from_secs(3)),
        }),
        BoardgameSite::ThePlayground => Box::new(ThePlaygroundScraper {}),
    }
}

async fn handle_message(scraper: &dyn Scraper, message: &Message) {
    let action = match message {
        Message::Discover() => scraper.discover(),
        Message::Update(product_info) => scraper.update(&product_info),
    };
    action.await
}

async fn listen_for_site(site: &BoardgameSite) -> Result<(), lapin::Error> {
    let scraper = get_scraper(site);
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

            handle_message(scraper.as_ref(), &message).await;

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
