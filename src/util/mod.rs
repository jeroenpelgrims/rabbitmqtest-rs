use std::sync::Arc;
use std::time::Duration;

use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use reqwest::Client;

#[derive(Clone)]
pub struct RateLimitedClient {
    client: Client,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
}

impl RateLimitedClient {
    pub fn new(throttle_duration: Duration) -> Self {
        let client = Client::new();
        let quota = Quota::with_period(throttle_duration).expect("throttle_duration must be set");
        let limiter = Arc::new(RateLimiter::direct(quota));
        Self { client, limiter }
    }

    pub async fn get(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        self.limiter.until_ready().await;
        self.client.get(url).send().await
    }
}
