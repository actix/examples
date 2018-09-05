use actix_web::error::Result;
use actix_web::middleware::session::RequestSession;
use actix_web::HttpRequest;

const FLASH_KEY: &str = "flash";

pub fn set_flash<T>(request: &HttpRequest<T>, flash: FlashMessage) -> Result<()> {
    request.session().set(FLASH_KEY, flash)
}

pub fn get_flash<T>(req: &HttpRequest<T>) -> Result<Option<FlashMessage>> {
    req.session().get::<FlashMessage>(FLASH_KEY)
}

pub fn clear_flash<T>(req: &HttpRequest<T>) {
    req.session().remove(FLASH_KEY);
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
