use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = {
        let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
        aws_sdk_dynamodb::Client::new(&config)
    };

    let app = backend::api::configure(client);

    #[cfg(debug_assertions)]
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    #[cfg(not(debug_assertions))]
    {
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        lambda_http::run(app).await.unwrap();
    }

    Ok(())
}
