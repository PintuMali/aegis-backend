use aws_sdk_s3::Client;

pub async fn create_gaming_bucket(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let bucket_name =
        std::env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "aegis-gaming-assets".to_string());

    println!("🔧 Testing MinIO connection...");

    match client.list_buckets().send().await {
        Ok(response) => {
            println!("✅ MinIO connection successful!");

            // Handle buckets properly
            match response.buckets {
                Some(ref buckets) => {
                    println!("📦 Available buckets:");
                    for bucket in buckets {
                        if let Some(name) = &bucket.name {
                            println!("  - {}", name);
                            if name == &bucket_name {
                                println!("✅ Found target bucket '{}'", bucket_name);
                                return Ok(());
                            }
                        }
                    }
                    println!("⚠️  Target bucket '{}' not found in list", bucket_name);
                }
                None => {
                    println!("📦 No buckets found");
                }
            }
        }
        Err(e) => {
            println!("❌ MinIO connection failed: {:?}", e);
            println!("🔍 Debug info:");
            println!(
                "   - Endpoint: {}",
                std::env::var("S3_ENDPOINT").unwrap_or_default()
            );
            println!(
                "   - Access Key: {}",
                std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_default()
            );
            println!(
                "   - Region: {}",
                std::env::var("AWS_REGION").unwrap_or_default()
            );
        }
    }

    println!("🎯 Continuing without bucket verification for development");
    Ok(())
}

pub async fn setup_bucket_policies(_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ S3 policies skipped for MinIO (development mode)");
    Ok(())
}
