#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260404_225136_roles;
mod m20260404_225800_goals;
mod m20260404_225817_rights;
mod m20260404_225854_create_join_table_goals_and_rights;
mod m20260404_230856_create_join_table_goal_rights_and_roles;
mod m20260404_231741_create_refresh_tokens;
mod m20260406_000901_populate_rights;
mod m20260406_001859_populate_goals;
mod m20260406_002231_populate_goal_rights;
mod m20260406_003215_populate_roles;
mod m20260406_003638_populate_goal_rights_roles;
mod m20260406_122756_create_join_table_users_and_roles;
mod m20260410_003534_create_event_types;
mod m20260410_003912_create_client_ids;
mod m20260410_004425_events;
mod m20260410_004554_create_value_types;
mod m20260410_004844_create_payload_types;
mod m20260410_005029_payloads;
mod m20260410_005410_create_allowed_payloads;
mod m20260410_010046_populate_value_types;
mod m20260410_010327_populate_payload_types;
mod m20260410_010626_populate_event_types;
mod m20260410_010919_populate_allowed_payloads;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260404_225136_roles::Migration),
            Box::new(m20260404_225800_goals::Migration),
            Box::new(m20260404_225817_rights::Migration),
            Box::new(m20260404_225854_create_join_table_goals_and_rights::Migration),
            Box::new(m20260404_230856_create_join_table_goal_rights_and_roles::Migration),
            Box::new(m20260404_231741_create_refresh_tokens::Migration),
            Box::new(m20260406_000901_populate_rights::Migration),
            Box::new(m20260406_001859_populate_goals::Migration),
            Box::new(m20260406_002231_populate_goal_rights::Migration),
            Box::new(m20260406_003215_populate_roles::Migration),
            Box::new(m20260406_003638_populate_goal_rights_roles::Migration),
            Box::new(m20260406_122756_create_join_table_users_and_roles::Migration),
            Box::new(m20260410_003534_create_event_types::Migration),
            Box::new(m20260410_003912_create_client_ids::Migration),
            Box::new(m20260410_004425_events::Migration),
            Box::new(m20260410_004554_create_value_types::Migration),
            Box::new(m20260410_004844_create_payload_types::Migration),
            Box::new(m20260410_005029_payloads::Migration),
            Box::new(m20260410_005410_create_allowed_payloads::Migration),
            Box::new(m20260410_010046_populate_value_types::Migration),
            Box::new(m20260410_010327_populate_payload_types::Migration),
            Box::new(m20260410_010626_populate_event_types::Migration),
            Box::new(m20260410_010919_populate_allowed_payloads::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
