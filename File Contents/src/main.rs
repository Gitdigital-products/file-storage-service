use axum::{
    extract::{Multipart, Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
struct FileMeta {
    id: String,
    filename: String,
    path: String,
}

type FileDb = Arc<Mutex<HashMap<String, FileMeta>>>;

#[tokio::main]
async fn main() {
    let db: FileDb = Arc::new(Mutex::new(HashMap::new()));
    fs::create_dir_all("uploads").unwrap();

    let app = Router::new()
        .route("/upload", post(upload_file))
        .route("/download/:id", get(download_file))
        .with_state(db.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4500));
    println!("📦 File Storage Service running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(db))
        .await
        .unwrap();
}

async fn upload_file(
    State(db): State<FileDb>,
    mut multipart: Multipart,
) -> Json<Vec<FileMeta>> {
    let mut results = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let bytes = field.bytes().await.unwrap();
        let id = Uuid::new_v4().to_string();
        let path = format!("uploads/{}_{}", id, file_name);

        fs::write(&path, &bytes).unwrap();

        let meta = FileMeta {
            id: id.clone(),
            filename: file_name.clone(),
            path: path.clone(),
        };

        db.lock().await.insert(id.clone(), meta.clone());
        results.push(meta);
    }

    Json(results)
}

async fn download_file(
    Path(id): Path<String>,
    State(db): State<FileDb>,
) -> Option<Vec<u8>> {
    let db = db.lock().await;
    if let Some(meta) = db.get(&id) {
        Some(fs::read(&meta.path).unwrap())
    } else {
        None
    }
}
