use actix_session::Session;
use actix_web::error::Result;
use serde::{Deserialize, Serialize};

const FLASH_KEY: &str = "flash";

pub fn set_flash(session: &Session, flash: FlashMessage) -> Result<()> {
    Ok(session.insert(FLASH_KEY, flash)?)
}

pub fn get_flash(session: &Session) -> Result<Option<FlashMessage>> {
    Ok(session.get::<FlashMessage>(FLASH_KEY)?)
}

pub fn clear_flash(session: &Session) {
    session.remove(FLASH_KEY);
}

#[derive(Deserialize, Serialize)]
pub struct FlashMessage {
    pub kind: String,
    pub message: String,
}

impl FlashMessage {
    pub fn success(message: &str) -> Self {
        Self {
            kind: "success".to_owned(),
            message: message.to_owned(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            kind: "error".to_owned(),
            message: message.to_owned(),
        }
    }
}
