use crate::models::postgres::{community, community_member, community_post};
use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

#[derive(Clone)]
pub struct CommunityService {
    db: DatabaseConnection,
}

impl CommunityService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_community(
        &self,
        name: String,
        description: String,
        community_type: String,
        owner: String,
    ) -> Result<String> {
        let community_id = Uuid::new_v4();
        let owner_uuid = Uuid::parse_str(&owner)?;

        let community = community::ActiveModel {
            id: Set(community_id),
            name: Set(name),
            slug: Set(format!("community-{}", community_id)),
            description: Set(description),
            privacy: Set(community_type),
            owner_id: Set(owner_uuid),
            moderators: Set(serde_json::json!([owner])),
            ..Default::default()
        };

        community.insert(&self.db).await?;
        Ok(community_id.to_string())
    }

    pub async fn get_community(&self, community_id: &str) -> Result<Option<community::Model>> {
        let community_uuid = Uuid::parse_str(community_id)?;
        Ok(community::Entity::find_by_id(community_uuid)
            .one(&self.db)
            .await?)
    }

    pub async fn add_post_to_community(
        &self,
        community_id: String,
        post_id: String,
        pinned: bool,
        added_by: String,
    ) -> Result<String> {
        let post_uuid = Uuid::new_v4();
        let community_uuid = Uuid::parse_str(&community_id)?;
        let author_uuid = Uuid::parse_str(&added_by)?;

        let community_post = community_post::ActiveModel {
            id: Set(post_uuid),
            community_id: Set(community_uuid),
            author_id: Set(author_uuid),
            title: Set(Some("Post".to_string())),
            content: Set(post_id),
            pinned: Set(pinned),
            ..Default::default()
        };

        community_post.insert(&self.db).await?;
        Ok(post_uuid.to_string())
    }

    pub async fn get_community_posts(
        &self,
        community_id: &str,
    ) -> Result<Vec<community_post::Model>> {
        let community_uuid = Uuid::parse_str(community_id)?;
        Ok(community_post::Entity::find()
            .filter(community_post::Column::CommunityId.eq(community_uuid))
            .all(&self.db)
            .await?)
    }

    pub async fn join_community(&self, community_id: &str, user_id: &str) -> Result<()> {
        let community_uuid = Uuid::parse_str(community_id)?;
        let user_uuid = Uuid::parse_str(user_id)?;

        let member = community_member::ActiveModel {
            id: Set(Uuid::new_v4()),
            community_id: Set(community_uuid),
            player_id: Set(user_uuid),
            role: Set("member".to_string()),
            ..Default::default()
        };

        member.insert(&self.db).await?;
        Ok(())
    }

    pub async fn leave_community(&self, community_id: &str, user_id: &str) -> Result<()> {
        let community_uuid = Uuid::parse_str(community_id)?;
        let user_uuid = Uuid::parse_str(user_id)?;

        community_member::Entity::delete_many()
            .filter(community_member::Column::CommunityId.eq(community_uuid))
            .filter(community_member::Column::PlayerId.eq(user_uuid))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_communities_by_owner(&self, owner_id: &str) -> Result<Vec<community::Model>> {
        let owner_uuid = Uuid::parse_str(owner_id)?;
        Ok(community::Entity::find()
            .filter(community::Column::OwnerId.eq(owner_uuid))
            .all(&self.db)
            .await?)
    }

    pub async fn get_community_members(
        &self,
        community_id: &str,
    ) -> Result<Vec<community_member::Model>> {
        let community_uuid = Uuid::parse_str(community_id)?;
        Ok(community_member::Entity::find()
            .filter(community_member::Column::CommunityId.eq(community_uuid))
            .all(&self.db)
            .await?)
    }
}
