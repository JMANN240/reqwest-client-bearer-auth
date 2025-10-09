use std::fmt::Display;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

fn get_bearer_auth_header_map<T: Display>(token: T) -> Result<HeaderMap, InvalidHeaderValue> {
    let mut authorization = HeaderValue::from_str(&format!("Bearer {token}"))?;
    authorization.set_sensitive(true);
    Ok(HeaderMap::from_iter([(reqwest::header::AUTHORIZATION, authorization)]))
}

pub trait BearerAuth {
    fn bearer_auth<T: Display>(self, token: T) -> Result<Self, InvalidHeaderValue> where Self: Sized;
}

impl BearerAuth for reqwest::ClientBuilder {
    fn bearer_auth<T: Display>(self, token: T) -> Result<Self, InvalidHeaderValue> {
        get_bearer_auth_header_map(token)
            .map(|bearer_auth_header_map| self.default_headers(bearer_auth_header_map))
    }
}

#[cfg(feature = "blocking")]
impl BearerAuth for reqwest::blocking::ClientBuilder {
    fn bearer_auth<T: Display>(self, token: T) -> Result<Self, InvalidHeaderValue> {
        get_bearer_auth_header_map(token)
            .map(|bearer_auth_header_map| self.default_headers(bearer_auth_header_map))
    }
}
