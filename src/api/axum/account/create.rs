use axum::http::StatusCode;

pub async fn handler() -> (StatusCode) {
    println!("Hello from create::handler()!");

    //TODO: Implement create::handler()!
    (StatusCode::CREATED)
}
