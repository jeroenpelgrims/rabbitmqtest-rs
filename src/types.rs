use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BoardgameSite {
    Spelonk,
    ThePlayground,
}

impl fmt::Display for BoardgameSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let site_str = match self {
            BoardgameSite::Spelonk => "spelonk",
            BoardgameSite::ThePlayground => "the-playground",
        };
        write!(f, "{}", site_str)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUpdateInfo {
    pub product_id: Uuid,
    pub product_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Discover(),
    Update(ProductUpdateInfo),
}
