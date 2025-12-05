use std::time::Duration;
use tokio::time::sleep;

pub struct MinioMonitor;

impl MinioMonitor {
    pub async fn monitor_health() {
        tokio::spawn(async {
            loop {
                match Self::check_minio_health().await {
                    Ok(healthy) => {
                        if !healthy {
                            tracing::warn!("🔄 MinIO unhealthy, waiting for recovery...");
                        } else {
                            tracing::debug!("✅ MinIO healthy");
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ MinIO health check failed: {}", e);
                    }
                }
                sleep(Duration::from_secs(30)).await;
            }
        });
    }

    async fn check_minio_health() -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("http://localhost:9000/minio/health/live") // MinIO health endpoint
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}
