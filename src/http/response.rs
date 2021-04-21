use super::StatusCode;
use crate::http::Headers;
use chrono::prelude::*;
use std::io::{Result as IOResult, Write};

#[derive(Debug)]
pub struct Response<'headers> {
    status_code: StatusCode,
    body: Option<String>,
    headers: Headers<'headers>,
}

impl<'headers> Response<'headers> {
    pub fn new_with_default_headers(status_code: StatusCode, body: Option<String>) -> Self {
        Self::new(status_code, body, Headers::new())
    }

    pub fn new(status_code: StatusCode, body: Option<String>, headers: Headers<'headers>) -> Self {
        Self {
            status_code,
            body,
            headers,
        }
    }

    pub fn add_header(&mut self, key: &'headers str, val: &'headers str) {
        self.headers.add(key, val)
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        let utc = Utc::now();
        let formatted_utc = utc.to_rfc2822();
        // TODO: what's the correct calculation for this?
        // let body_len = (body.len() + 1).to_string();

        let mut default_headers = Headers::new();
        default_headers.add("Date", formatted_utc.as_str());
        default_headers.add("Server", "eli's rusty server");
        // default_headers.add("Content-Length", &body_len);

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            default_headers,
            &self.headers,
            body,
        )
    }
}
