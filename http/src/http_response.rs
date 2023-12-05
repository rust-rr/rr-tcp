use std::{collections::HashMap, fmt::Write, io::Result};

#[derive(Debug, PartialEq, Clone)]
pub struct Response<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Response {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<Response<'a>> for String {
    fn from(res: Response) -> Self {
        format!(
            "{} {} {}\r\n{}Content Length: {}\r\n\r\n{}",
            res.version,
            res.status_code,
            res.status_text,
            res.headers(),
            res.body().len(),
            res.body(),
        )
    }
}

impl<'a> Response<'a> {
    fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Response<'a> {
        let mut response: Response<'a> = Response::default();

        if status_code != "200" {
            response.status_code = status_code;
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Unknow Error",
        };

        response.body = body;

        response
    }

    fn send_response(&self, stream: &mut impl Write) -> Result<()> {
        let res = self.clone();

        let res_string = String::from(res);
        let _ = write!(stream, "{}", res_string);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut headers_string = String::new();

        for (k, v) in map.iter() {
            headers_string.push_str(&format!("{}:{}\r\n", k, v));
        }

        headers_string
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b,
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_default() {
        let response: Response = Response::default();

        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status_code, "200");
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.headers, None);
        assert_eq!(response.body, None);
    }

    #[test]
    fn test_response_new() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/html");

        let response: Response = Response::new("404", Some(headers), Some("Not Found".to_string()));

        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status_code, "404");
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(
            response.headers.unwrap().get("Content-Type"),
            Some(&"text/html")
        );
        assert_eq!(response.body, Some("Not Found".to_string()));
    }

    #[test]
    fn test_response_into_string() {
        let response: Response = Response::new("200", None, Some("xxxx".to_string()));
        let response_str: String = response.into();

        assert_eq!(
            response_str,
            "HTTP/1.1 200 OK\r\nContent-Type:text/html\r\nContent Length: 4\r\n\r\nxxxx"
        );
    }

    // TODO: send_response
}
