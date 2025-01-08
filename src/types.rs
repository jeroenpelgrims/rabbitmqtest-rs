use std::fmt;

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
