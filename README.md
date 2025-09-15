# 📦 http-echo

A lightweight web server built in **Rust** using [Actix-Web](https://actix.rs/) and [Clap](https://docs.rs/clap) for command-line argument parsing.  
It is designed for testing and experimenting with simple server behaviors.
It can run in two modes:  

- **echo** → echoes the request path, headers, method, scheme and host.  
- **static** → always returns a fixed text.  

Logging is configurable, and the server can be containerized and deployed for multiple architectures.

---

## 🚀 Features

- **Two execution modes**:  
  - `echo` → returns request path, headers, and metadata.  
  - `static` → returns a static text configured at startup.  
- **Configurable log level** (`off`, `error`, `warn`, `info`, `debug`, `trace`).  
- **Customizable host and port**.  
- **Built with Actix-Web** → high-performance async web server.  
- **Lightweight Docker image with multi-architecture support** (`amd64`, `arm`, `riscv64`, etc.).  

---

## ⚙️ CLI Arguments

```bash
Usage: myapp [OPTIONS]

Options:
      --mode <MODE>                Execution mode (echo, static) [default: echo]
      --log-level <LOG_LEVEL>      Log level of the server (off, error, warn, info, debug, trace) [default: info]
      --static-text <TEXT>         Text returned by the server if execution mode is static [default: "Hello World!"]
      --host <HOST>                Server host [default: 0.0.0.0]
      --port <PORT>                Server port [default: 8080]
  -V, --version                    Print version
  -h, --help                       Print help
```

---

## 🛠️ Build & Run

### Build locally
```bash
cargo build --release
```

### Run
```bash
http-echo --mode static --static-text "Hi from Rust!" --port 8080
```

## 🌍 Examples

**Static mode:**
```bash
http-echo --mode static --static-text "Hello World!" --port 8080
```

```bash
curl http://localhost:8080/
```

Response:
```text
Hello World!
```

**Echo mode:**

```bash
http-echo --mode echo --port 8080
```

```bash
curl -H "X-Custom: test" http://localhost:8080/hello
```

Response:
```json
{
  "headers": {
    "accept":"*/*",
    "host":"localhost:8080",
    "user-agent":"curl/8.7.1",
    "x-custom":"test"
  },
  "host":"localhost:8080",
  "method":"GET",
  "path":"/hello",
  "scheme":"http"
}
```

---

## 📜 License

MIT License. See [LICENSE](LICENSE) for details.
