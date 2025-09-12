# file-storage-service
file-storage-service[package] name = "file-storage-service" version = "0.1.0" edition = "2021"  [dependencies] tokio = { version = "1", features = ["full"] } axum = { version = "0.7", features = ["multipart"] } serde = { version = "1.0", features = ["derive"] } serde_json = "1.0" uuid = { version = "1", features = ["v4"] } tokio-util = "0.7"
