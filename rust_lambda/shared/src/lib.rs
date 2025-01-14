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
