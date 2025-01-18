use async_trait::async_trait;

use super::Scraper;
mod discover;
mod update;
use crate::types::ProductUpdateInfo;
use crate::util::RateLimitedClient;

pub struct SpelonkScraper {
    pub client: RateLimitedClient,
}

#[async_trait]
impl Scraper for SpelonkScraper {
    async fn discover(&self) {
        let response = self.client.get("https://jeroenpelgrims.com").await.unwrap();
        let text = response.text().await.unwrap();
        println!("{}", text);
    }

    async fn update(&self, product_info: &ProductUpdateInfo) {
        println!("Updating data for spelonk, {:?}", product_info);
    }
}
