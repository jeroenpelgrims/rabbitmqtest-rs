use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};

use crate::types::BoardgameSite;

const EXCHANGE: &str = "boardgamefinder";

pub async fn setup_exchange_and_queue(
    channel: &Channel,
    site: BoardgameSite,
) -> Result<(), lapin::Error> {
    let site_name = site.to_string();

    channel
        .exchange_declare(
            EXCHANGE,
            lapin::ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            &site_name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            &site_name,
            EXCHANGE,
            &site_name,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}
