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
use std::{ops::Add, sync::Arc};

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/increment", post(increment))
        .with_state(Arc::new(model::Model::new()))
}

async fn hello(State(model): State<Arc<Model>>) -> Html<String> {
    Html(views::greet("World", &model.score.read().unwrap()))
}

async fn increment(State(model): State<Arc<Model>>) -> Html<String> {
    let mut x = model.score.write().unwrap();
    *x += 1;
    Html(views::counter(&x))
}
