use std::time::Duration;

use async_trait::async_trait;

// mod scrapers;
// mod types;
// use scrapers::Scraper;
// use types::ProductUpdateInfo;

// Enum to represent different websites
enum Website {
    SiteA,
    SiteB,
}

// Struct for Website A
struct SiteAScraper {
    throttle_ms: i32,
}

#[async_trait]
impl Scraper for SiteAScraper {
    async fn discover(&self) {
        println!(
            "Discovering data for Site A, throttle_ms: {}",
            self.throttle_ms
        );
        // Add async discovery logic for Site A
    }

    async fn update(&self, foo: &ProductUpdateInfo) {
        println!("Updating data for Site A");
        // Add async update logic for Site A
    }
}

// Struct for Website B
struct SiteBScraper;

#[async_trait]
impl Scraper for SiteBScraper {
    async fn discover(&self) {
        println!("Discovering data for Site B");
        // Add async discovery logic for Site B
    }

    async fn update(&self, foo: &ProductUpdateInfo) {
        println!("Updating data for Site B");
        // Add async update logic for Site B
    }
}

// Function to create the appropriate scraper based on the enum
fn get_scraper(website: Website) -> Box<dyn Scraper + Send + Sync> {
    match website {
        Website::SiteA => Box::new(SiteAScraper { throttle_ms: 123 }),
        Website::SiteB => Box::new(SiteBScraper),
    }
}

#[tokio::main]
async fn main() {
    let website = Website::SiteA;

    let scraper = get_scraper(website);

    scraper.discover().await;
}
