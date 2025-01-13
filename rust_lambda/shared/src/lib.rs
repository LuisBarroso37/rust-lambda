use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;

pub async fn get_dynamodb_client() -> Client {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    Client::new(&config)
}
