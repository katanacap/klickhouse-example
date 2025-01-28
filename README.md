# Klickhouse Example + Actix Web 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/Rust-1.83%2B-blue.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Actix%20Web-4.9-brightgreen.svg)](https://actix.rs/)
[![ClickHouse](https://img.shields.io/badge/ClickHouse-24.3.7%2B-orange.svg)](https://clickhouse.com/)
![CI Status](https://img.shields.io/badge/build-passing-brightgreen)

A sample **Actix Web** application demonstrating integration with **ClickHouse** using the [klickhouse](https://github.com/katanacap/klickhouse) library. This example covers essential aspects of building a web service in Rust, including routing, middleware, and database connectivity.

---

## Features ✨

1. **Actix Web**  
   - Route handling (e.g., `GET`, `POST`)
   - Custom middleware for logging and error handling

2. **ClickHouse Integration**  
   - Reading from and writing to ClickHouse
   - Example table (`web_server_logs`) to store and retrieve logs

3. **Logging Middleware**  
   - Captures HTTP request/response details and saves them into ClickHouse

4. **Error & Panic Handling**  
   - Demonstrates capturing errors and panics within the Actix Web middleware stack

---

## Getting Started

### Prerequisites 🛠️
- **Devenv** - https://devenv.sh/ (for running the application under development environment)

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/klickhouse-example.git
   cd klickhouse-example
   ```

2. **Run the application**:
   ```bash
   direnv allow
   devenv up // for running clickhouse as a service
   cargo run migrate // to run clickhouse migrations
   cargo run serve // to run web the server
   ```

3. **Access the application**:
   - Open your browser and navigate to `http://localhost:1337/health` to check if the application is running.
   - Open your browser and navigate to `http://localhost:1337/logs?limit=10&offset=0` to view the logs.
  
4. **Run tests + all rust checks**:
   ```bash
   // cargo test/fmt/clippy/audit/outdated
   just
   // only tests
   just test
   ```


### API Endpoints 📡

| Endpoint     | Method	  | Description                   | Example                             |
|--------------|----------|-------------------------------|-------------------------------------|
| `/`          | GET      | Demo endpoint with request ID | `curl http://localhost:1337`        |
| `/health`    | GET      | Healthcheck                   | `curl http://localhost:1337/health` |
| `/logs`      | GET      | Get paginated logs            | `curl "http://localhost:1337/logs?limit=5&offset=0"` |
| `/fail`      | GET      | Trigger handle panic example  | `curl http://localhost:1337/fail` |

### CLI Commands 🛠️
- `cargo run migrate`: Run ClickHouse migrations.
- `cargo run serve`: Run the web server.

### Configuration 🛠️
- `confik.toml`: Configuration file for the application.
- `.env`: Environment variables for the application (optional).

### Docker build 🐳
```shell
# x86_64
docker build -t klickhouse-example .

# ARM64
docker build -t klickhouse-example -f aarch64.Dockerfile .
```

## Contributing 🤝

Feel free to open issues or PRs to improve this example — whether adding features or refining best practices. All suggestions are welcome!

## License 📄

This project is open-sourced under the MIT License - see the [LICENSE](LICENSE) file for details.
