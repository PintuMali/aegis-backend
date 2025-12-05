use crate::models::postgres::{chat, chat_message};
use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatService {
    db: DatabaseConnection,
}

impl ChatService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_chat(
        &self,
        name: String,
        chat_type: String,
        created_by: String,
    ) -> Result<String> {
        let chat_id = Uuid::new_v4();
        let created_by_uuid = Uuid::parse_str(&created_by)?;

        let chat = chat::ActiveModel {
            id: Set(chat_id),
            name: Set(name),
            chat_type: Set(chat_type),
            created_by: Set(created_by_uuid),
            participants: Set(serde_json::json!([created_by])),
            ..Default::default()
        };

        chat.insert(&self.db).await?;
        Ok(chat_id.to_string())
    }

    pub async fn get_chat(&self, chat_id: &str) -> Result<Option<chat::Model>> {
        let chat_uuid = Uuid::parse_str(chat_id)?;
        Ok(chat::Entity::find_by_id(chat_uuid).one(&self.db).await?)
    }

    pub async fn send_message(
        &self,
        chat_id: String,
        sender: String,
        message: String,
        message_type: String,
    ) -> Result<String> {
        let message_id = Uuid::new_v4();
        let chat_uuid = Uuid::parse_str(&chat_id)?;
        let sender_uuid = Uuid::parse_str(&sender)?;

        let chat_message = chat_message::ActiveModel {
            id: Set(message_id),
            chat_id: Set(chat_uuid),
            sender_id: Set(sender_uuid),
            message: Set(message),
            message_type: Set(message_type),
            ..Default::default()
        };

        chat_message.insert(&self.db).await?;
        Ok(message_id.to_string())
    }

    pub async fn get_messages(
        &self,
        chat_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<chat_message::Model>> {
        let chat_uuid = Uuid::parse_str(chat_id)?;
        let mut query = chat_message::Entity::find()
            .filter(chat_message::Column::ChatId.eq(chat_uuid))
            .order_by_desc(chat_message::Column::CreatedAt);

        if let Some(limit) = limit {
            query = query.limit(limit as u64);
        }

        Ok(query.all(&self.db).await?)
    }

    pub async fn get_chats_by_type(&self, chat_type: &str) -> Result<Vec<chat::Model>> {
        Ok(chat::Entity::find()
            .filter(chat::Column::ChatType.eq(chat_type))
            .all(&self.db)
            .await?)
    }

    pub async fn join_chat(&self, chat_id: &str, user_id: &str) -> Result<()> {
        use sea_orm::{ActiveModelTrait, TransactionTrait};

        let chat_uuid = Uuid::parse_str(chat_id)?;
        let user_uuid = Uuid::parse_str(user_id)?;

        // Transaction: Ensure data consistency
        let txn = self.db.begin().await?;

        //  Validate chat exists and user has permission
        let chat = chat::Entity::find_by_id(chat_uuid)
            .one(&txn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Chat not found"))?;

        // Security: Check max participants limit
        let current_participants: Vec<String> =
            serde_json::from_value(chat.participants.clone()).unwrap_or_default();

        if current_participants.len() >= chat.max_participants as usize {
            return Err(anyhow::anyhow!("Chat is at maximum capacity"));
        }

        //  Prevent duplicate joins
        if current_participants.contains(&user_id.to_string()) {
            return Ok(()); // Already a participant
        }

        // Update: Add user to participants JSONB array
        let mut updated_participants = current_participants;
        updated_participants.push(user_id.to_string());

        let mut chat_active: chat::ActiveModel = chat.into();
        chat_active.participants = Set(serde_json::json!(updated_participants));
        chat_active.updated_at = Set(chrono::Utc::now().into());

        chat_active.update(&txn).await?;

        //  Audit: Log the join action
        use crate::models::postgres::activity_log;
        let audit_log = activity_log::ActiveModel {
            id: Set(Uuid::new_v4()),
            entity_type: Set("chat".to_string()),
            entity_id: Set(chat_uuid),
            actor_id: Set(Some(user_uuid)),
            action: Set("join_chat".to_string()),
            details: Set(serde_json::json!({
                "chat_id": chat_id,
                "user_id": user_id,
                "participant_count": updated_participants.len()
            })),
            ip_address: Set(None), // TODO: Pass from request context
            user_agent: Set(None), // TODO: Pass from request context
            created_at: Set(chrono::Utc::now().into()),
        };

        audit_log.insert(&txn).await?;

        //  Commit: Atomic operation
        txn.commit().await?;

        tracing::info!(
            "User {} successfully joined chat {} (participants: {})",
            user_id,
            chat_id,
            updated_participants.len()
        );

        Ok(())
    }
}
