use axum::{middleware, Extension, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use crate::middleware::auth;
use crate::{handler::{auth:: auth_handler,  file_query::get_file_list_handler, files::file_handle, user::users_handler}, AppState};



pub fn create_router(app_state: Arc<AppState>) -> Router {
  let api_route = Router::new()
  .nest("/auth", auth_handler())
  .nest(
    "/users",
    users_handler()
    .layer(middleware::from_fn(auth))
  )
  .nest(
    "/file",
    file_handle()
    .layer(middleware::from_fn(auth))
  )
  .nest(
    "/list",
    get_file_list_handler()
    .layer(middleware::from_fn(auth))
  )
  .layer(TraceLayer::new_for_http())
  .layer(Extension(app_state));

  Router::new().nest("/api", api_route)
}