use std::path::{Component, Path, PathBuf};

use axum::body::Body;
use axum::extract::State;
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use leptos::config::LeptosOptions;

use crate::server::state::AppState;

pub async fn site_assets(State(state): State<AppState>, uri: Uri) -> Response {
    let Some(path) = resolve_site_asset_path(&state.inner.leptos_options, uri.path()) else {
        return not_found();
    };

    serve_asset(path).await
}

async fn serve_asset(path: PathBuf) -> Response {
    let bytes = match tokio::fs::read(&path).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return not_found(),
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read asset: {error}"),
            )
                .into_response();
        }
    };

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(bytes))
        .unwrap_or_else(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to build asset response",
            )
                .into_response()
        })
}

fn resolve_site_asset_path(options: &LeptosOptions, uri_path: &str) -> Option<PathBuf> {
    let path = uri_path.trim_start_matches('/');
    if path.is_empty() {
        return None;
    }

    let mut relative = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => relative.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }

    Some(Path::new(options.site_root.as_ref()).join(relative))
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "not found").into_response()
}
