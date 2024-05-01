use core::str;
use std::{cmp::Ordering, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tokio::fs;
use tower_http::services::ServeDir;
use tracing::info;

struct HttpState {
    path: PathBuf,
}

struct Filename {
    name: String,
    is_dir: bool,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr_str = format!("{}:{}", "0.0.0.0", port);
    let sock_addr = SocketAddr::from_str(&addr_str)?;
    info!("Listening on {}", &addr_str);

    let state = HttpState {
        path: PathBuf::from(&dir),
    };
    let router = axum::Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        // NOTE: uri `/` is ignored
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(sock_addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpState>>,
    Path(url_filename): Path<String>,
) -> Response<Body> {
    // NOTE directory traversal is processed correctly by Axum route
    let fullpath = std::path::Path::new(&state.path).join(&url_filename);
    info!("access: {}", fullpath.display());

    if !fullpath.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("not found: {}", fullpath.display()),
        )
            .into_response();
    }

    if fullpath.is_dir() {
        render_html(fullpath, &url_filename)
            .await
            .map(Html)
            .map_err(|e| e.to_string())
            .into_response()
    } else {
        match fs::read(&fullpath).await {
            Ok(content) => match str::from_utf8(&content) {
                Ok(_) => unsafe { String::from_utf8_unchecked(content) }.into_response(),
                Err(_) => content.into_response(),
            },
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

async fn render_html(path: PathBuf, basepath: &str) -> Result<String> {
    let mut paths = list_dir(path).await?;
    paths.sort();
    let mut list = Vec::with_capacity(paths.len());

    for name in paths.iter() {
        let link = name.try_into_html_link(basepath);
        let li = format!("<li>{}</li>", link?);
        list.push(li);
    }

    let result = format!(
        "<html>\n<body>\n<ul>\n{}</ul>\n</body>\n</html>\n",
        list.join("\n")
    );

    Ok(result)
}

async fn list_dir(path: PathBuf) -> Result<Vec<Filename>> {
    let mut reader = fs::read_dir(path).await?;
    let mut paths = Vec::new();

    while let Some(entry) = reader.next_entry().await? {
        let name = match Filename::new(entry.path()) {
            Some(file) => file,
            None => return Err(anyhow!("invalid characters in path")),
        };
        paths.push(name);
    }

    Ok(paths)
}

impl Filename {
    fn try_into_html_link(&self, basepath: &str) -> Result<String> {
        if self.is_dir {
            Ok(format!(
                "<a href=\"/{}/{}\">{}/</a>",
                basepath.trim_end_matches('/'),
                self.name,
                self.name,
            ))
        } else {
            Ok(format!(
                "<a href=\"/{}/{}\">{}</a>",
                basepath.trim_end_matches('/'),
                self.name,
                self.name,
            ))
        }
    }
}

impl Filename {
    fn new(path: PathBuf) -> Option<Self> {
        // terminates in '..' or invalid unicode characters
        Some(Self {
            name: path.file_name()?.to_str()?.to_string(),
            is_dir: path.is_dir(),
        })
    }
}

impl PartialEq for Filename {
    fn eq(&self, other: &Self) -> bool {
        self.is_dir == other.is_dir && self.name.eq(&other.name)
    }
}

impl PartialOrd for Filename {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Filename {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Filename {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_dir, other.is_dir) {
            (true, true) => self.name.cmp(&other.name),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::body::HttpBody;

    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpState {
            path: PathBuf::from_str(".").expect("path error"),
        });
        let path = "Cargo.toml".to_string();
        let resp = file_handler(State(state), Path(path.clone())).await;
        assert!(resp.status().is_success());
        let size = resp.body().size_hint().exact().unwrap();
        assert!(size > 100);
    }

    #[tokio::test]
    async fn test_file_handler_binary() {
        let state = Arc::new(HttpState {
            path: PathBuf::from_str(".").expect("path error"),
        });
        let path = "fixtures/ed25519.pk".to_string();
        let resp = file_handler(State(state), Path(path.clone())).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers()["content-type"], "application/octet-stream");
        let size = resp.body().size_hint().exact().unwrap();
        assert!(size > 0);
    }

    #[tokio::test]
    async fn directory_index() {
        let state = Arc::new(HttpState {
            path: PathBuf::from_str(".").expect("path error"),
        });
        let resp = file_handler(State(state), Path("fixtures".to_string())).await;
        assert!(resp.status().is_success());
        assert!(resp.headers()["content-type"]
            .to_str()
            .unwrap()
            .contains("text/html"));
        let size = resp.body().size_hint().exact().unwrap();
        assert!(size > 0);
    }

    // test Filename::into_html_link
}
