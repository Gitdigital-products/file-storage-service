# file-storage-service
file-storage-service[package] name = "file-storage-service" version = "0.1.0" edition = "2021"  [dependencies] tokio = { version = "1", features = ["full"] } axum = { version = "0.7", features = ["multipart"] } serde = { version = "1.0", features = ["derive"] } serde_json = "1.0" uuid = { version = "1", features = ["v4"] } tokio-util = "0.7"
# File Storage Service

Handles file uploads + downloads in the **Gitdigital Products** ecosystem.  
Supports avatars, documents, media, and general file storage.

## 🚀 Features
- `POST /upload` → Upload files (multipart form-data)
- `GET /download/:id` → Download by file ID
- Files stored locally in `/uploads`
- Metadata tracked in memory

## 🛠️ Setup
```bash
cargo run
