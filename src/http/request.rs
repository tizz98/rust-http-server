use super::method::{Method, MethodError};
use super::{Headers, QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: Option<Headers<'buf>>,
    body: Option<&'buf str>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn headers(&self) -> Option<&Headers> {
        self.headers.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n<HEADERS>\r\n\r\n<BODY>
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        let first_line_end = request.find("\r\n").ok_or(ParseError::InvalidRequest)?;
        let first_line = &request[..first_line_end];

        let (method, first_line) = get_next_word(first_line).ok_or(ParseError::InvalidRequest)?;
        let (mut path, first_line) = get_next_word(first_line).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(first_line).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let mut headers = None;
        if let Some(i) = request.find("\r\n") {
            headers = Some(Headers::from(&request[i + 2..]));
        }

        let mut body = None;
        if let Some(i) = request.find("\r\n\r\n") {
            let end_index = request.find('\u{0}').unwrap_or(request.len() - 1);
            body = Some(&request[i + 4..end_index]);
        }

        Ok(Self {
            path,
            query_string,
            method,
            headers,
            body,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        } else if i == request.len() - 1 {
            return Some((&request[..i + 1], ""));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
