use std::{io, error::Error};

use tokio_tungstenite::tungstenite;




#[derive(Debug)]
pub enum ShoutcastError {
    IoError(io::Error),
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
}

impl From<io::Error> for ShoutcastError {
    fn from(err: io::Error) -> ShoutcastError {
        ShoutcastError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for ShoutcastError {
    fn from(err: std::num::ParseIntError) -> ShoutcastError {
        ShoutcastError::ParseIntError(err)
    }
}

impl From<std::str::Utf8Error> for ShoutcastError {
    fn from(err: std::str::Utf8Error) -> ShoutcastError {
        ShoutcastError::Utf8Error(err)
    }
}

impl std::fmt::Display for ShoutcastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ShoutcastError {}

// #[derive(Debug)]
// pub enum RockradioError {
//     UrlParse(url::ParseError),
//     Connection(tungstenite::Error),
//     JsonParse(serde_json::Error),
//     MessageConversion(tungstenite::error::MessageError),
// }

// impl From<url::ParseError> for RockradioError {
//     fn from(err: url::ParseError) -> RockradioError {
//         RockradioError::UrlParse(err)
//     }
// }

// impl From<tungstenite::Error> for RockradioError {
//     fn from(err: tungstenite::Error) -> RockradioError {
//         RockradioError::Connection(err)
//     }
// }

// impl From<serde_json::Error> for RockradioError {
//     fn from(err: serde_json::Error) -> RockradioError {
//         RockradioError::JsonParse(err)
//     }
// }

// impl From<tungstenite::error::MessageError> for RockradioError {
//     fn from(err: tungstenite::error::MessageError) -> RockradioError {
//         RockradioError::MessageConversion(err)
//     }
// }
