use async_trait::async_trait;

pub mod spelonk;
pub mod the_playground;

use crate::types::ProductUpdateInfo;

pub enum ScraperError {
    Foo(String),
}

#[async_trait]
pub trait Scraper {
    async fn discover(&self);
    async fn update(&self, product_info: &ProductUpdateInfo);
}
