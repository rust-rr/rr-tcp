use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut parse_method = Method::Uninitialized;
        let mut parse_version = Version::V1_1;
        let mut parse_resource = Resource::Path("".to_string());
        let mut parse_headers = HashMap::new();
        let mut parse_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_request_line(line);
                parse_method = method;
                parse_version = version;
                parse_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parse_headers.insert(key, value);
            } else if line.is_empty() {
            } else {
                parse_msg_body = line;
            }
        }

        Request {
            method: parse_method,
            version: parse_version,
            resource: parse_resource,
            headers: parse_headers,
            msg_body: parse_msg_body.to_string(),
        }
    }
}

fn process_request_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap().into();
    let resource = Resource::Path(words.next().unwrap().to_string());
    let version = words.next().unwrap().into();

    (method, resource, version)
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::from("GET"), Method::Get);
        assert_eq!(Method::from("POST"), Method::Post);
        assert_eq!(Method::from("FOO"), Method::Uninitialized);
    }

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_from_str() {
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("FOO"), Version::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let v1: Version = "HTTP/1.1".into();
        assert_eq!(v1, Version::V1_1);

        let v2: Version = "HTTP/2.0".into();
        assert_eq!(v2, Version::V2_0);
    }

    #[test]
    fn test_request_from_string() {
        let s: String =
            String::from("GET /greeting HTTP/1.1\r\nHost:example.com\r\nAccept:*/*\r\n\r\n");
        let request: Request = s.into();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.resource, Resource::Path("/greeting".to_string()));
        assert_eq!(request.headers.get("Host"), Some(&"example.com".into()));
        assert_eq!(request.headers.get("Accept"), Some(&"*/*".into()));
        assert_eq!(request.msg_body, "");
    }
}
