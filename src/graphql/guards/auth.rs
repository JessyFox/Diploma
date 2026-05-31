use async_graphql::dynamic::ResolverContext;
use seaography::{GuardAction, LifecycleHooksInterface, OperationType};

use crate::{
    common::constants::{
        CREATE_RIGHT_ID, DELETE_RIGHT_ID, EDIT_RIGHT_ID, GRANT_RIGHT_ID, ROLES_GOAL_ID,
        USERS_GOAL_ID, VIEW_RIGHT_ID,
    },
    services::permissions::RequestPermissions,
};

pub struct AuthGuard {}

impl Default for AuthGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthGuard {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl LifecycleHooksInterface for AuthGuard {
    fn entity_guard(
        &self,
        ctx: &ResolverContext<'_>,
        entity: &str,
        action: OperationType,
    ) -> GuardAction {
        tracing::debug!(entity, "Olala");
        let goal_id = match entity {
            "Users" => USERS_GOAL_ID,
            _ => return GuardAction::Allow,
        };
        let right_id = if goal_id == ROLES_GOAL_ID && entity == "UserRoles" {
            GRANT_RIGHT_ID
        } else {
            match action {
                OperationType::Read => VIEW_RIGHT_ID,
                OperationType::Create => CREATE_RIGHT_ID,
                OperationType::Update => EDIT_RIGHT_ID,
                OperationType::Delete => DELETE_RIGHT_ID,
            }
        };
        let perms: &RequestPermissions = match ctx.data() {
            Ok(perms) => perms,
            Err(e) => {
                tracing::debug!("Can't access permissions: {}", e.message);
                return GuardAction::Block(None);
            }
        };
        if perms.has(goal_id, right_id) {
            GuardAction::Allow
        } else {
            GuardAction::Block(Some("User has no rights for this action".to_string()))
        }
    }
}
