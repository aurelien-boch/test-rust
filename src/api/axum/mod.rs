use axum::Router;

mod account;

pub fn router() -> Router
{
     Router::new()
         .nest("/account", account::router())
}
