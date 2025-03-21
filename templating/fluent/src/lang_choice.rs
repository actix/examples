use std::{
    convert::Infallible,
    future::{Ready, ready},
};

use actix_web::{FromRequest, HttpMessage as _, HttpRequest, dev, http::header::AcceptLanguage};
use fluent_templates::LanguageIdentifier;
use serde::Serialize;

/// A convenient extractor that finds the clients's preferred language based on an Accept-Language
/// header and falls back to English if header is not found. Serializes easily in Handlebars data.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct LangChoice(String);

impl LangChoice {
    pub(crate) fn from_req(req: &HttpRequest) -> Self {
        let lang = req
            .get_header::<AcceptLanguage>()
            .and_then(|lang| lang.preference().into_item())
            .map_or_else(|| "en".to_owned(), |lang| lang.to_string());

        Self(lang)
    }

    pub fn lang_id(&self) -> LanguageIdentifier {
        // unwrap: lang ID should be valid given extraction method
        self.0.parse().unwrap()
    }
}

impl FromRequest for LangChoice {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        ready(Ok(Self::from_req(req)))
    }
}
