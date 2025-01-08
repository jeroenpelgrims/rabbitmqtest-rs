use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};

use crate::types::BoardgameSite;

pub const EXCHANGE: &str = "boardgamefinder";

pub async fn get_connection() -> Result<Connection, lapin::Error> {
    let url = "amqp://rabbitmq:rabbitmq@localhost:5672";
    let conn = Connection::connect(url, ConnectionProperties::default()).await?;
    Ok(conn)
}

pub async fn ensure_exchange(channel: &Channel) -> Result<(), lapin::Error> {
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
    Ok(())
}

pub async fn ensure_queue(channel: &Channel, site: &BoardgameSite) -> Result<(), lapin::Error> {
    let site_name = site.to_string();

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
