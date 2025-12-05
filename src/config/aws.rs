use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client as S3Client;
use std::env;

#[derive(Clone)]
pub struct AwsClients {
    pub s3: S3Client,
}

impl AwsClients {
    pub async fn new() -> Self {
        let config = if is_local_development() {
            let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
            let endpoint =
                env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string());

            // Add explicit credentials for MinIO
            let credentials = Credentials::new(
                env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "minioadmin".to_string()),
                env::var("AWS_SECRET_ACCESS_KEY")
                    .unwrap_or_else(|_| "minioadmin@12345".to_string()),
                None,
                None,
                "minio",
            );

            aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(region))
                .endpoint_url(endpoint)
                .credentials_provider(credentials)
                .load()
                .await
        } else {
            let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
            aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(region))
                .load()
                .await
        };

        Self {
            s3: S3Client::new(&config),
        }
    }
}

fn is_local_development() -> bool {
    env::var("S3_ENDPOINT")
        .unwrap_or_default()
        .contains("localhost")
}
