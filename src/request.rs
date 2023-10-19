use super::method::Method;
use super::query::Query;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter, Result as FmtResult};

// POST /health?pickaboo=bamm HTTP/1.1
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
}

pub enum ParseError {
    InvalidProtocol,
    InvalidMethod,
    InvalidRequest,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidMethod => write!(f, "{}", "Invalid Method"),
            Self::InvalidProtocol => return write!(f, "{}", "Invalid Protocol"),
            Self::InvalidRequest => return write!(f, "{}", "Invalid Request"),
        }
    }
}

impl TryFrom<String> for Request {
    type Error = ParseError;

    fn try_from(request: String) -> Result<Self, Self::Error> {
        let (method, request) = get_request_segment(&request).ok_or(ParseError::InvalidRequest)?;
        let method = method.parse().or(Err(ParseError::InvalidMethod))?;

        let (path, request) = get_request_segment(&request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _request) =
            get_request_segment(&request).ok_or(ParseError::InvalidRequest)?;

        let route: &str;
        let query_string: Option<&str>;

        if let Some(i) = path.find("?") {
            route = &path[..i];
            query_string = Some(&path[(i + 1)..]);
        } else {
            route = path;
            query_string = None;
        }
        println!("route: {}, query string {:?}", route, query_string);

        let query_map = Query::from(query_string);

        println!("{:?}", query_map.query);

        if !protocol.contains("HTTP/1.1") {
            return Err(ParseError::InvalidProtocol);
        }


        Ok(Request {
            method,
            path: path.to_string(),
            protocol: protocol.to_string(),
        })
    }
}

fn get_request_segment(request_line: &str) -> Option<(&str, &str)> {
    match request_line.find(" ") {
        Some(space_index) => {
            let segment = &request_line[..space_index];
            // using (space_index+1) cause space is a one byte character
            let rest = &request_line[(space_index + 1)..];

            return Some((segment, rest));
        }
        None => return None,
    }
}
