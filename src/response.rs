pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}


impl Response {
    pub fn new(status_code: u16, status_text: &str, body: Vec<u8>, content_type: Option<&str>) -> Self {
        let mut headers = vec![
            ("Content-Length".to_string(), body.len().to_string()),
            ("Server".to_string(), "RustHTTPServer/0.1".to_string()),
        ];

        if let Some(ct) = content_type {
            headers.push(("Content-Type".to_string(), ct.to_string()));
        }

        Response {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body,
        }
    }
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }

    pub fn ok(body: Vec<u8>, content_type: Option<&str>) -> Self {
        Self::new(200, "OK", body, content_type)
    }

    pub fn not_found() -> Self {
        let body = b"404 Not Found".to_vec();
        Self::new(404, "Not Found", body, Some("text/plain"))
    }

    pub fn forbidden() -> Self {
        let body = b"403 Forbidden".to_vec();
        Self::new(403, "Forbidden", body, Some("text/plain"))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();

        // Status line
        response.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text).as_bytes());

        // Headers
        for (name, value) in &self.headers {
            response.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }

        // Empty line separating headers from body
        response.extend_from_slice(b"\r\n");

        // Body
        response.extend_from_slice(&self.body);

        response
    }
}
