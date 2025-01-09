mod mq;
mod types;

use lapin::options::BasicPublishOptions;
use lapin::BasicProperties;
use mq::{ensure_exchange, ensure_queue, get_connection, EXCHANGE};
use tokio;
use types::{BoardgameSite, Message, ProductUpdateInfo};

#[tokio::main]
async fn main() -> Result<(), lapin::Error> {
    let conn = get_connection().await?;
    let channel = conn.create_channel().await?;
    ensure_exchange(&channel).await?;
    ensure_queue(&channel, &BoardgameSite::Spelonk).await?;
    ensure_queue(&channel, &BoardgameSite::ThePlayground).await?;

    let message = Message::Update(ProductUpdateInfo {
        product_id: uuid::Uuid::new_v4(),
        product_url: "http://google.com/product.html".to_string(),
    });
    // let message = Message::Discover();
    let payload = serde_json::to_vec(&message).unwrap();

    channel
        .basic_publish(
            EXCHANGE,
            &BoardgameSite::ThePlayground.to_string(),
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?
        .await?;

    println!("Message published!");

    Ok(())
}
