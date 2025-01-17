use super::ScraperError;
use crate::types::Message;
mod discover;
mod update;

pub async fn handle(message: &Message) -> Result<(), ScraperError> {
    match message {
        Message::Discover() => discover::discover().await,
        Message::Update(info) => update::update(info).await,
    }
}
