use std::fmt;

use serde::{Deserialize, Serialize};

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
pub struct Message {
    pub msg_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoverMessage {
    pub msg_type: String,
    pub foo: String,
}
