use async_trait::async_trait;

use super::Scraper;
use crate::types::ProductUpdateInfo;

pub struct ThePlaygroundScraper {}

#[async_trait]
impl Scraper for ThePlaygroundScraper {
    async fn discover(&self) {}

    async fn update(&self, product_info: &ProductUpdateInfo) {
        todo!()
    }
}
