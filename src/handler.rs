use crate::response::Response;

/// A parsed HTTP request — just method + path, which is all routing needs.
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
}

impl<'a> Request<'a> {
    /// Parse the request line: "GET /hello HTTP/1.1"
    ///
    /// We only look at the first line — the rest is headers we don't need yet.
    /// Returns None if the request is malformed.
    pub fn parse(raw: &'a str) -> Option<Self> {
        let first_line = raw.lines().next()?;    // "GET /hello HTTP/1.1"
        let mut parts = first_line.split_whitespace();
        let method = parts.next()?;              // "GET"
        let path   = parts.next()?;              // "/hello"
        Some(Self { method, path })
    }
}

/// Route a request to a response.
///
/// The match arms are (method, path) tuples. Any unrecognised combination
/// falls through to the wildcard arm and gets a 404.
pub fn handle(req: Request<'_>) -> Response {
    match (req.method, req.path) {
        ("GET", "/")      => home(),
        ("GET", "/hello") => hello(),
        _                 => Response::not_found(),
    }
}

// --- route handlers ---------------------------------------------------------

fn home() -> Response {
    Response::ok("\
Welcome to gaddy.

Routes:
  GET /        → this page
  GET /hello   → greeting
")
}

fn hello() -> Response {
    Response::ok("Hello! You reached /hello.\n")
}
