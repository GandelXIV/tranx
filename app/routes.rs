use crate::{
    model::{self, Model},
    views,
};
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/increment", post(increment))
        .with_state(Arc::new(model::Model::new()))
}

async fn hello(State(model): State<Arc<Model>>) -> Html<String> {
    Html(views::greet("World", &model.get_score()))
}

async fn increment(State(model): State<Arc<Model>>) -> Html<String> {
    model.increment_score();
    Html(views::counter(&model.get_score()))
}
