use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PATCH,
    PUT,
    DELETE,
    CONNECT,
}

pub struct MethodError;

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),

            _ => Err(MethodError),
        }
    }
}