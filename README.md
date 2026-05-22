# gaddy

A minimal async HTTP server built from scratch in Rust, written to understand how things work at the wire level — no frameworks, no middleware, just TCP sockets and bytes.

Built in phases. Each phase adds one concept.

---

## Phases

### Phase 1 — TCP + raw HTTP
Accept TCP connections and write a hand-crafted HTTP response string.
**Concepts:** TCP listener, async runtime, HTTP structure.

### Phase 2 — Basic Routing ← current
Parse the request line, extract the path, match routes.
**Concepts:** request parsing, routing logic.

---

## Routes

| Method | Path     | Response         |
|--------|----------|------------------|
| GET    | `/`      | home + route list |
| GET    | `/hello` | greeting         |
| *      | *        | 404 Not Found    |

---

## Run

```bash
cargo run
```

```bash
curl http://localhost:7878/
curl http://localhost:7878/hello
curl http://localhost:7878/nope   # → 404
```

## Project layout

```
src/
  main.rs       # boots the tokio runtime
  server.rs     # TCP listener loop, spawns tasks per connection
  handler.rs    # parses requests, matches routes
  response.rs   # builds raw HTTP/1.1 response bytes
```

## Stack

- [tokio](https://tokio.rs) — async runtime
