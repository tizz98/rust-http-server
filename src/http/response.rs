use super::StatusCode;
use crate::http::Headers;
use chrono::prelude::*;
use std::io::{Result as IOResult, Write};

#[derive(Debug)]
pub struct Response<'headers> {
    status_code: StatusCode,
    body: Option<String>,
    headers: Option<Headers<'headers>>,
}

impl<'headers> Response<'headers> {
    pub fn new_with_default_headers(status_code: StatusCode, body: Option<String>) -> Self {
        Self::new(status_code, body, Some(Headers::new()))
    }

    pub fn new(
        status_code: StatusCode,
        body: Option<String>,
        headers: Option<Headers<'headers>>,
    ) -> Self {
        Self {
            status_code,
            body,
            headers,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // todo: copy headers and then mutate
        // let headers = self.headers.unwrap_or(Headers::new());
        let mut headers = Headers::new();
        let utc = Utc::now();
        let formatted_utc = utc.to_rfc2822();
        let body_len = (body.len() + 1).to_string();

        headers.add("Date", formatted_utc.as_str());
        headers.add("Server", "eli's rusty server");
        headers.add("Content-Length", &body_len);
        // TODO: Content-Type

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            headers, // TODO: use real headers
            body
        )
    }
}
