use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sender_id: Uuid,
    pub receiver_id: Option<Uuid>, // None for group chats
    pub room_id: Option<String>,   // For tournament/team chats
    pub message: String,
    pub message_type: MessageType,
    pub timestamp: DateTime,
    pub edited: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Text,
    Image,
    File,
    System,
}
