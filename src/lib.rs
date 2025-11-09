pub mod config;
pub mod handlers;
pub mod middleware;
pub mod migration;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod scripts;
pub mod services;
pub mod utils;

use repositories::{ChatRepository, CommunityRepository, DynamoRepository, PostRepository};
use services::{AuthService, ChatService, CommunityService, PlayerService, PostService, S3Service};
// ... rest of your code

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub aws: config::AwsClients,
    pub settings: config::Settings,
    pub auth_service: AuthService,
    pub player_service: PlayerService,
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
        let auth_service = AuthService::new(settings.jwt.secret.clone(), settings.jwt.expiration);
        let player_service = PlayerService::new(db.clone());
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
            auth_service,
            player_service,
            chat_service,
            post_service,
            community_service,
            s3_service,
        }
    }
}
