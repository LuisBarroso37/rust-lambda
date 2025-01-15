use aws_config::{BehaviorVersion, Region, SdkConfig};
use tracing_subscriber::fmt::format::FmtSpan;

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
        .with_current_span(false)
        .with_ansi(false)
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();
}
