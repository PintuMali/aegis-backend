use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathPermission {
    pub path: String,
    pub access: Vec<String>,
    pub require_verified: Option<bool>,
    pub description: Option<String>,
}

pub fn get_path_permissions() -> Vec<PathPermission> {
    vec![
        // Auth routes (public)
        PathPermission {
            path: "/auth/login".to_string(),
            access: vec!["public".to_string()],
            require_verified: None,
            description: Some("User login".to_string()),
        },
        PathPermission {
            path: "/auth/register".to_string(),
            access: vec!["public".to_string()],
            require_verified: None,
            description: Some("User registration".to_string()),
        },
        // Admin routes
        PathPermission {
            path: "/admin/*".to_string(),
            access: vec!["admin".to_string()],
            require_verified: Some(true),
            description: Some("Admin panel access".to_string()),
        },
        // Player routes
        PathPermission {
            path: "/players".to_string(),
            access: vec!["admin".to_string(), "player".to_string()],
            require_verified: Some(true),
            description: Some("Player list access".to_string()),
        },
        PathPermission {
            path: "/players/me".to_string(),
            access: vec!["player".to_string()],
            require_verified: Some(true),
            description: Some("Player profile access".to_string()),
        },
        PathPermission {
            path: "/players/*".to_string(),
            access: vec!["admin".to_string(), "player".to_string()],
            require_verified: Some(true),
            description: Some("Player management".to_string()),
        },
        // Organization routes
        PathPermission {
            path: "/organizations/*".to_string(),
            access: vec!["admin".to_string(), "organization".to_string()],
            require_verified: Some(true),
            description: Some("Organization management".to_string()),
        },
        // Tournament routes
        PathPermission {
            path: "/tournaments/*".to_string(),
            access: vec![
                "admin".to_string(),
                "player".to_string(),
                "organization".to_string(),
            ],
            require_verified: Some(true),
            description: Some("Tournament access".to_string()),
        },
        // Chat routes
        PathPermission {
            path: "/chats/*".to_string(),
            access: vec![
                "admin".to_string(),
                "player".to_string(),
                "organization".to_string(),
            ],
            require_verified: Some(true),
            description: Some("Chat system".to_string()),
        },
        // Community routes
        PathPermission {
            path: "/communities/*".to_string(),
            access: vec![
                "admin".to_string(),
                "player".to_string(),
                "organization".to_string(),
            ],
            require_verified: Some(false),
            description: Some("Community features".to_string()),
        },
        // Upload routes
        PathPermission {
            path: "/uploads/*".to_string(),
            access: vec![
                "admin".to_string(),
                "player".to_string(),
                "organization".to_string(),
            ],
            require_verified: Some(true),
            description: Some("File uploads".to_string()),
        },
    ]
}
