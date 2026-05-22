/// A minimal HTTP/1.1 response builder.
pub struct Response {
    status_code: u16,
    status_text: &'static str,
    content_type: &'static str,
    body: String,
}

impl Response {
    pub fn ok(body: impl Into<String>) -> Self {
        Self {
            status_code: 200,
            status_text: "OK",
            content_type: "text/plain",
            body: body.into(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            status_text: "Not Found",
            content_type: "text/plain",
            body: "404 Not Found".to_string(),
        }
    }

    /// Serialize to a raw HTTP/1.1 byte string ready to write to the socket.
    pub fn into_bytes(self) -> Vec<u8> {
        let body = self.body;
        let header = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.status_code,
            self.status_text,
            self.content_type,
            body.len(),
        );
        let mut bytes = header.into_bytes();
        bytes.extend_from_slice(body.as_bytes());
        bytes
    }
}
