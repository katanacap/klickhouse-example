# Actix Web + Klickhouse Example

A sample **Actix Web** application demonstrating integration with **ClickHouse** using the [klickhouse](https://github.com/katanacap/klickhouse) library. This example covers essential aspects of building a web service in Rust, including routing, middleware, and database connectivity.

---

## Features

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

### Prerequisites
- **Devenv** (for running the application under development environment)

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


### Endpoints:
- GET /: example endpoint to check request id logic
- GET /logs: Fetch logs from ClickHouse (optional query params for pagination).
- GET /health: Basic health check.
- GET /fail: Example endpoint that triggers an error or panic for demonstration.

### CLI Commands
- `cargo run migrate`: Run ClickHouse migrations.
- `cargo run serve`: Run the web server.

### Configuration
- `confik.toml`: Configuration file for the application.
- `.env`: Environment variables for the application (optional).

### Docker build
```shell
docker build -t klickhouse_example .
```

### Docker build for aarch64
```shell
docker build -t klickhouse-example-img -f aarch64.Dockerfile .
```

## Contributing

Feel free to open issues or PRs to improve this example — whether adding features or refining best practices. All suggestions are welcome!

## License

This project is open-sourced under the MIT License - see the [LICENSE](LICENSE) file for details.
