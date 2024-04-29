use std::{net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tokio::fs;
use tower_http::services::ServeDir;
use tracing::info;

struct HttpState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr_str = format!("{}:{}", "0.0.0.0", port);
    let sock_addr = SocketAddr::from_str(&addr_str)?;
    info!("Listening on {}", &addr_str);

    let state = HttpState {
        path: PathBuf::from(&dir),
    };
    let router = axum::Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(sock_addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", p.display()),
        )
    } else {
        match fs::read_to_string(p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpState {
            path: PathBuf::from_str(".").expect("path error"),
        });
        let path = "Cargo.toml".to_string();

        let (code, content) = file_handler(State(state), Path(path)).await;

        assert!(code == StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
