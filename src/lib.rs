pub mod config;
pub mod handlers;
pub mod models;
pub mod services;
pub mod middleware;
pub mod utils;
pub mod routes;
pub mod migration;
pub mod repositories;
pub mod scripts;

use repositories::{DynamoRepository, ChatRepository, PostRepository, CommunityRepository};
use services::{ChatService, PostService, CommunityService, S3Service};

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub aws: config::AwsClients,
    pub settings: config::Settings,
    pub chat_service: ChatService,
    pub post_service: PostService,
    pub community_service: CommunityService,
    pub s3_service: S3Service,
}

impl AppState {
    pub async fn new(
        db: sea_orm::DatabaseConnection,
        aws: config::AwsClients,
        settings: config::Settings,
    ) -> Self {
        let dynamo_repo = DynamoRepository::new(aws.dynamodb.clone());
        let chat_repo = ChatRepository::new(dynamo_repo.clone());
        let post_repo = PostRepository::new(dynamo_repo.clone());
        let community_repo = CommunityRepository::new(dynamo_repo);

        let chat_service = ChatService::new(chat_repo);
        let post_service = PostService::new(post_repo);
        let community_service = CommunityService::new(community_repo);
        let s3_service = S3Service::new(aws.s3.clone());

        Self {
            db,
            aws,
            settings,
            chat_service,
            post_service,
            community_service,
            s3_service,
        }
    }
}
