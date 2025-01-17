use std::time::Duration;

use async_trait::async_trait;
mod util;
use util::RateLimitedClient;

// Enum to represent different websites
enum Website {
    SiteA,
    SiteB,
}

// Trait to define scraping behavior
#[async_trait]
trait Scraper {
    async fn discover(&self);
    async fn update(&self);
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

    async fn update(&self) {
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

    async fn update(&self) {
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
    // Example: Enum value received for the website
    let website = Website::SiteA;

    // Get the appropriate scraper implementation
    let scraper = get_scraper(website);

    // Call discover or update as needed
    scraper.discover().await;
    scraper.update().await;

    let client = RateLimitedClient::new(Duration::from_secs(3));
}
