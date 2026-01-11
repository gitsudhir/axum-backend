# Axum Backend

A minimal and fast web backend built with [Axum](https://github.com/tokio-rs/axum), a web framework built on top of [Tokio](https://tokio.rs/) and [Hyper](https://hyper.rs/).

## Live Demo

Check out the deployed application: [https://axum-backend-gvpq.onrender.com](https://axum-backend-gvpq.onrender.com)

## Features

- Built with Rust for memory safety and performance
- Uses Axum framework for routing and middleware
- Async runtime powered by Tokio
- Configurable port via environment variables
- IPv6/IPv4 dual-stack support

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd axum-backend
```

2. Build and run the application:
```bash
cargo run
```

3. The server will start on `http://localhost:3000` by default

### Environment Variables

- `PORT`: Port number to run the server on (defaults to 3000)

## Project Structure

```
src/
├── main.rs          # Main application entry point
```

## Endpoints

- `GET /` - Returns "✅ Hello World, from Axum! 🚀"

## Deployment

This project is configured for deployment on platforms like Render, Heroku, or other cloud providers that support Rust applications.

## Dependencies

- `axum` - Web framework
- `tokio` - Async runtime with full feature set

## License

This project is open source and available under the [MIT License](LICENSE).