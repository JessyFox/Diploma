use insta::assert_debug_snapshot;
use loco_rs::auth::jwt::JWT;
use loco_rs::testing::prelude::*;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serial_test::serial;
use stat_api_rs::extractors::refresh_jwt::get_refresh_from_config;
use stat_api_rs::{
    app::App,
    models::{
        refresh_tokens::{ActiveModel, Model},
        users,
    },
};
use std::str::FromStr;
use uuid::Uuid;

fn cleanup_refresh_token_model() -> Vec<(&'static str, &'static str)> {
    let mut filters = vec![
        (r"[^_]id: \d+,", " id: ID"),
        (
            r"jti: ([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})",
            "jti: PID",
        ),
    ];
    filters.extend(get_cleanup_date().iter().copied());
    filters
}

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("refresh_tokens");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_can_save_refresh_token() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let user =
        users::Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
            .await
            .expect("Failed to find user");

    let refresh_token: ActiveModel = ActiveModel::builder()
        .set_user_id(user.id)
        .set_jti(Uuid::new_v4())
        .set_expired_at(DateTimeWithTimeZone::from(
            chrono::Utc::now() + chrono::Duration::days(7),
        ))
        .set_browser(Some("Chrome 12.0".to_string()))
        .set_user_ip(Some("127.0.0.1".to_string()))
        .set_revoked(false)
        .into();

    let res = refresh_token.insert(&boot.app_context.db).await;

    insta::with_settings!({
        filters => cleanup_refresh_token_model()
    }, {
       assert_debug_snapshot!(res);
    });
}

#[tokio::test]
#[serial]
async fn test_can_find_by_jti() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");
    let jti_res = Uuid::from_str("11111111-1111-1111-1111-111111111111");
    assert!(jti_res.is_ok(), "Should be valid uuid");
    let jti = jti_res.unwrap();
    let res = Model::find_by_jti(&boot.app_context.db, jti).await;

    assert_debug_snapshot!(res);
}

#[tokio::test]
#[serial]
async fn test_can_revoke_token() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");
    let jti_res = Uuid::from_str("11111111-1111-1111-1111-111111111111");
    assert!(jti_res.is_ok(), "Should be valid uuid");
    let jti = jti_res.unwrap();
    let model = Model::find_by_jti(&boot.app_context.db, jti)
        .await
        .expect("Model should be in database");

    let res = model
        .into_active_model()
        .revoke_token(&boot.app_context.db)
        .await;

    insta::with_settings!({
        filters => get_cleanup_date().clone()
    }, {
       assert_debug_snapshot!(res);
    });
}

#[tokio::test]
#[serial]
async fn test_can_rotate_refresh_token() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");
    let jti_res = Uuid::from_str("11111111-1111-1111-1111-111111111111");
    assert!(jti_res.is_ok(), "Should be valid uuid");
    let jti = jti_res.unwrap();
    let model = Model::find_by_jti(&boot.app_context.db, jti)
        .await
        .expect("Model should be in database");

    let res = model
        .into_active_model()
        .rotate_refresh_token(&boot.app_context.db)
        .await;

    insta::with_settings!({
        filters => cleanup_refresh_token_model()
    }, {
       assert_debug_snapshot!(res);
    });

    let res = Model::find_by_jti(&boot.app_context.db, jti).await;

    insta::with_settings!({
        filters => get_cleanup_date().clone()
    }, {
       assert_debug_snapshot!(res);
    });
}

#[tokio::test]
#[serial]
async fn test_generate_refresh_token_cookie() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let user =
        users::Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
            .await
            .expect("Failed to find user");

    let jti_res = Uuid::from_str("11111111-1111-1111-1111-111111111111");
    assert!(jti_res.is_ok(), "Should be valid uuid");
    let jti = jti_res.unwrap();
    let token = Model::find_by_jti(&boot.app_context.db, jti)
        .await
        .expect("Model should be in database");
    let refresh_config = get_refresh_from_config(&boot.app_context)
        .expect("Refresh token section should be set in config");
    let cookie = token
        .generate_refresh_token(
            &refresh_config.secret,
            refresh_config.expiration,
            &refresh_config.name,
            &user.pid,
        )
        .expect("Cookie should generate without errors");
    let token = JWT::new(&refresh_config.secret).validate(cookie.value());

    insta::with_settings!({
        filters => vec![
            ("exp: [0-9]{10}", "exp: EXPIRED_AT")
        ]
    }, {
       assert_debug_snapshot!(token);
    });
}

#[tokio::test]
#[serial]
async fn test_before_save_sets_jti_on_insert() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let user =
        users::Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
            .await
            .expect("Failed to find user");

    // Create ActiveModel without explicitly setting jti
    let refresh_token: ActiveModel = ActiveModel::builder()
        .set_user_id(user.id)
        .set_expired_at(DateTimeWithTimeZone::from(
            chrono::Utc::now() + chrono::Duration::days(7),
        ))
        .set_browser(Some("Brave".to_string()))
        .set_user_ip(Some("10.10.10.1".to_string()))
        .set_revoked(false)
        .into();

    insta::with_settings!({
        filters => get_cleanup_date().clone()
    }, {
       assert_debug_snapshot!(refresh_token);
    });

    let saved_token = refresh_token.insert(&boot.app_context.db).await;

    insta::with_settings!({
        filters => cleanup_refresh_token_model()
    }, {
       assert_debug_snapshot!(saved_token);
    });
}
