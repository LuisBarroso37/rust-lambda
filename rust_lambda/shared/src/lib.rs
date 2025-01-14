use aws_config::{BehaviorVersion, Region, SdkConfig};

pub mod test;

pub async fn get_aws_config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("eu-west-1"))
        .load()
        .await
}

pub fn get_required_env_variable(env_variable: &str) -> String {
    std::env::var(env_variable).unwrap_or_else(|_| {
        panic!(
            "Environment variable {env_variable} must be set",
            env_variable = env_variable
        )
    })
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        // This needs to be set to remove duplicated information in the log.
        .with_current_span(false)
        // This needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // Disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // Remove the name of the function from every log entry.
        .with_target(false)
        .init();
}
