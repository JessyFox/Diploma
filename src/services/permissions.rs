use crate::{
    common::constants::{get_goal_by_name, get_right_by_name},
    models::{
        _entities::{
            goal_right_roles as _goal_right_roles, goal_rights as _goal_rights,
            user_roles as _user_roles, roles as _roles,
        },
        goal_rights,
    },
};
use loco_rs::{
    cache::Cache,
    prelude::{Error, Result},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::Duration};

#[derive(
    FromQueryResult, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize,
)]
pub struct GoalRight {
    pub goal_id: i32,
    pub right_id: i32,
}

pub struct PermissionService {}

impl PermissionService {
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    /// Gathers user permissions from database
    ///
    /// # Errors
    /// Database error or cache error
    pub async fn get_user_permissions(
        &self,
        db: &DatabaseConnection,
        cache: &Cache,
        user_id: i64,
    ) -> Result<HashSet<GoalRight>> {
        {
            let value = cache
                .get::<HashSet<GoalRight>>(&format!("user:{user_id}"))
                .await?;
            if let Some(perms) = value {
                return Ok(perms);
            }
        }

        let permissions: HashSet<GoalRight> = goal_rights::Entity::find()
            .select_only()
            .column(_goal_rights::Column::GoalId)
            .column(_goal_rights::Column::RightId)
            .join(
                JoinType::InnerJoin,
                _goal_rights::Entity::has_many(_goal_right_roles::Entity).into(),
            )
            .join(
                JoinType::InnerJoin,
                _goal_right_roles::Relation::Roles.def(),
            )
            .join(JoinType::InnerJoin, _roles::Entity::has_many(_user_roles::Entity).into())
            .filter(_user_roles::Column::UserId.eq(user_id))
            .distinct()
            .into_model::<GoalRight>()
            .all(db)
            .await?
            .into_iter()
            .collect();

        {
            cache
                .insert_with_expiry(
                    &format!("user:{user_id}"),
                    &permissions,
                    Duration::from_secs(300),
                )
                .await?;
        }
        Ok(permissions)
    }

    /// Gathers user permissions from database
    ///
    /// # Errors
    /// Database error or cache error or some actions is absent in builtin dict
    pub async fn check_permission(
        &self,
        db: &DatabaseConnection,
        cache: &Cache,
        user_id: i64,
        resource: &str,
        actions: &[&str],
    ) -> Result<bool> {
        let goal_by_name = get_goal_by_name();
        let right_by_name = get_right_by_name();
        let &goal_id = goal_by_name
            .get(resource)
            .ok_or(Error::InternalServerError)?;
        let required: HashSet<GoalRight> = actions
            .iter()
            .filter_map(|&action| {
                right_by_name
                    .get(action)
                    .map(|&right_id| GoalRight { goal_id, right_id })
            })
            .collect();
        if required.len() != actions.len() {
            return Err(Error::InternalServerError);
        }

        let perms = self.get_user_permissions(db, cache, user_id).await?;
        Ok(required.is_subset(&perms))
    }
}

#[derive(Clone, Debug)]
pub struct RequestPermissions {
    pub user_id: i64,
    pub rights: HashSet<GoalRight>,
}

impl RequestPermissions {
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn new(user_id: i64, rights: HashSet<GoalRight>) -> Self {
        Self { user_id, rights }
    }

    #[inline]
    #[must_use]
    pub fn has(&self, goal_id: i32, right_id: i32) -> bool {
        self.rights.contains(&GoalRight { goal_id, right_id })
    }

    #[must_use]
    pub fn has_any(&self, goal_id: i32, right_ids: &[i32]) -> bool {
        let set: HashSet<GoalRight> = right_ids
            .iter()
            .map(|&right_id| GoalRight { goal_id, right_id })
            .collect();
        set.is_subset(&self.rights)
    }
}
