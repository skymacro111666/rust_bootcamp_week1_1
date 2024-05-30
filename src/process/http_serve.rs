use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);
    let state = HttpServerState { path: path.clone() };
    let dir_service = ServeDir::new(path);
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // format!("{:?},{}", state, path)
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        warn!("File does not exist");
        (
            StatusCode::NOT_FOUND,
            format!("File {} does not exist", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Reading {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
