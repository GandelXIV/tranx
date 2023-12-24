use crate::{
    context,
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
use tera::Context;

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/increment", post(increment))
        .with_state(Arc::new(model::Model::new()))
}

// Here go routes

async fn hello(State(model): State<Arc<Model>>) -> Html<String> {
    views::render_template(
        "greet.html",
        &context! { name => "World", score => &model.get_score()},
    )
}

async fn increment(State(model): State<Arc<Model>>) -> Html<String> {
    model.increment_score();
    views::render_template("counter.html", &context! { score => &model.get_score() })
}
