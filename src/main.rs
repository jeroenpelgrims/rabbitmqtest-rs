use std::fmt;

mod mq;
mod types;
use types::BoardgameSite;

use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), lapin::Error> {
    let conn = Connection::connect(
        "amqp://rabbitmq:rabbitmq@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;

    mq::setup_exchange_and_queue(&channel, BoardgameSite::Spelonk).await?;
    mq::setup_exchange_and_queue(&channel, BoardgameSite::ThePlayground).await?;

    let payload = b"Hello, RabbitMQ!";
    channel
        .basic_publish(
            "my_exchange",
            &BoardgameSite::Spelonk.to_string(),
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await?
        .await?;

    println!("Message published!");

    Ok(())
}
