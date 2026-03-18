use crate::web::routes;
use crate::web::state::AppState;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub struct WebServer {
    state: AppState,
    port: u16,
}

impl WebServer {
    pub fn new(state: AppState, port: u16) -> Self {
        Self { state, port }
    }

    pub async fn start(self) -> Result<()> {
        let app = self.create_router();

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        tracing::info!("Web server starting on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    fn create_router(&self) -> Router {
        Router::new()
            // API routes
            .route("/api/accounts", get(routes::accounts::list_accounts))
            .route("/api/accounts", post(routes::accounts::create_account))
            .route("/api/accounts/:id", get(routes::accounts::get_account))
            .route("/api/accounts/:id", post(routes::accounts::update_account))
            .route("/api/accounts/:id", axum::routing::delete(routes::accounts::delete_account))

            .route("/api/tasks", get(routes::tasks::list_tasks))
            .route("/api/tasks", post(routes::tasks::create_task))
            .route("/api/tasks/:id", get(routes::tasks::get_task))
            .route("/api/tasks/:id/cancel", post(routes::tasks::cancel_task))

            .route("/api/contents", get(routes::contents::list_contents))
            .route("/api/contents", post(routes::contents::create_content))
            .route("/api/contents/:id", get(routes::contents::get_content))
            .route("/api/contents/:id", axum::routing::delete(routes::contents::delete_content))

            .route("/api/stats", get(routes::stats::get_stats))
            .route("/api/status", get(routes::stats::get_status))

            // WebSocket
            .route("/ws", get(routes::websocket::ws_handler))

            // Static files
            .nest_service("/", ServeDir::new("web-ui"))

            // Share state
            .with_state(self.state.clone())
    }
}
