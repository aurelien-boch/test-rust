use std::net::SocketAddr;

mod configuration;
mod app;
mod api;

fn main()
{
    let ctx = configuration::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            axum::Server::bind(&addr)
                .serve(
                    api::axum::router()
                        .into_make_service()
                )
                .await
                .unwrap();
        });
}
