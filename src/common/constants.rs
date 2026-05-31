use std::{collections::HashMap, sync::OnceLock};

// Константы целей

// Имена целей
pub static STATS_GOAL_NAME: &str = "stats";
pub static DASHBOARDS_GOAL_NAME: &str = "dashboards";
pub static USERS_GOAL_NAME: &str = "users";
pub static ROLES_GOAL_NAME: &str = "roles";
pub static TABLES_GOAL_NAME: &str = "tables";
pub static RESOURCES_GOAL_NAME: &str = "resources";
pub static TASKS_GOAL_NAME: &str = "tasks";
pub static NAV_GOAL_NAME: &str = "nav_data";
pub static USER_PASS_GOAL_NAME: &str = "user_pass";
pub static ADMIN_GOAL_NAME: &str = "admin";
pub static REVIEWS_GOAL_NAME: &str = "reviews";

// ID целей
pub static STATS_GOAL_ID: i32 = 1;
pub static DASHBOARDS_GOAL_ID: i32 = 2;
pub static USERS_GOAL_ID: i32 = 3;
pub static ROLES_GOAL_ID: i32 = 4;
pub static TABLES_GOAL_ID: i32 = 5;
pub static RESOURCES_GOAL_ID: i32 = 6;
pub static TASKS_GOAL_ID: i32 = 7;
pub static NAV_GOAL_ID: i32 = 8;
pub static USER_PASS_GOAL_ID: i32 = 9;
pub static ADMIN_GOAL_ID: i32 = 10;
pub static REVIEWS_GOAL_ID: i32 = 11;

// Константы прав

// Имена прав
pub static VIEW_RIGHT_NAME: &str = "view";
pub static CREATE_RIGHT_NAME: &str = "create";
pub static EDIT_RIGHT_NAME: &str = "edit";
pub static DELETE_RIGHT_NAME: &str = "delete";
pub static GRANT_RIGHT_NAME: &str = "grant";

// ID прав
pub static VIEW_RIGHT_ID: i32 = 1;
pub static CREATE_RIGHT_ID: i32 = 2;
pub static EDIT_RIGHT_ID: i32 = 3;
pub static DELETE_RIGHT_ID: i32 = 4;
pub static GRANT_RIGHT_ID: i32 = 5;

// Словари маппинга

static GOAL_BY_ID: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
static RIGHT_BY_ID: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
static GOAL_BY_NAME: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
static RIGHT_BY_NAME: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();

pub fn get_goal_by_id() -> &'static HashMap<i32, &'static str> {
    GOAL_BY_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(STATS_GOAL_ID, STATS_GOAL_NAME);
        map.insert(DASHBOARDS_GOAL_ID, DASHBOARDS_GOAL_NAME);
        map.insert(USERS_GOAL_ID, USERS_GOAL_NAME);
        map.insert(ROLES_GOAL_ID, ROLES_GOAL_NAME);
        map.insert(TABLES_GOAL_ID, TABLES_GOAL_NAME);
        map.insert(RESOURCES_GOAL_ID, RESOURCES_GOAL_NAME);
        map.insert(TASKS_GOAL_ID, TASKS_GOAL_NAME);
        map.insert(NAV_GOAL_ID, NAV_GOAL_NAME);
        map.insert(USER_PASS_GOAL_ID, USER_PASS_GOAL_NAME);
        map.insert(ADMIN_GOAL_ID, ADMIN_GOAL_NAME);
        map.insert(RESOURCES_GOAL_ID, RESOURCES_GOAL_NAME);
        map
    })
}

pub fn get_right_by_id() -> &'static HashMap<i32, &'static str> {
    RIGHT_BY_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(VIEW_RIGHT_ID, VIEW_RIGHT_NAME);
        map.insert(CREATE_RIGHT_ID, CREATE_RIGHT_NAME);
        map.insert(EDIT_RIGHT_ID, EDIT_RIGHT_NAME);
        map.insert(DELETE_RIGHT_ID, DELETE_RIGHT_NAME);
        map.insert(GRANT_RIGHT_ID, GRANT_RIGHT_NAME);
        map
    })
}

pub fn get_goal_by_name() -> &'static HashMap<&'static str, i32> {
    GOAL_BY_NAME.get_or_init(|| get_goal_by_id().iter().map(|(&k, &v)| (v, k)).collect())
}

pub fn get_right_by_name() -> &'static HashMap<&'static str, i32> {
    RIGHT_BY_NAME.get_or_init(|| get_right_by_id().iter().map(|(&k, &v)| (v, k)).collect())
}
