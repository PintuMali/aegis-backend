use crate::config::permissions::{get_path_permissions, PathPermission};
use crate::services::auth_service::Claims;
use axum::http::StatusCode;
use std::sync::OnceLock;

static PERMISSIONS: OnceLock<Vec<PathPermission>> = OnceLock::new();

fn get_permissions() -> &'static Vec<PathPermission> {
    PERMISSIONS.get_or_init(|| get_path_permissions())
}

pub fn check_permission(path: &str, claims: &Claims) -> Result<(), StatusCode> {
    let permissions = get_permissions();

    // Find matching permission (longest match wins)
    let matching_permission = permissions
        .iter()
        .filter(|perm| path_matches(path, &perm.path))
        .max_by_key(|perm| perm.path.len());

    if let Some(permission) = matching_permission {
        // Check if public route
        if permission.access.contains(&"public".to_string()) {
            return Ok(());
        }

        // Check role access
        if !permission.access.contains(&claims.user_type) {
            return Err(StatusCode::FORBIDDEN);
        }

        // Check verification requirement
        if permission.require_verified.unwrap_or(false) && !claims.verified {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        // No permission defined = forbidden
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}

fn path_matches(request_path: &str, permission_path: &str) -> bool {
    if permission_path.ends_with("*") {
        let prefix = &permission_path[..permission_path.len() - 1];
        request_path.starts_with(prefix)
    } else {
        request_path == permission_path
    }
}
