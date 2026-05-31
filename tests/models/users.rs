use insta::assert_debug_snapshot;
use loco_rs::{model::Authenticable, testing::prelude::*};
use sea_orm::IntoActiveModel;
use serial_test::serial;
use stat_api_rs::{
    app::App,
    models::users::{self, Model},
};

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("users");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_can_validate_model() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let res = users::ActiveModel::builder()
        .set_name("1".to_string())
        .set_email("invalid-email".to_string())
        .set_login("1".to_string())
        .set_is_active(true)
        .insert(&boot.app_context.db)
        .await;

    assert_debug_snapshot!(res);
}

#[tokio::test]
#[serial]
async fn can_find_by_email() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let existing_user = Model::find_by_email(&boot.app_context.db, "user1@example.com").await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "un@existing-email.com").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[tokio::test]
#[serial]
async fn can_find_by_login() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let existing_user = Model::find_by_login(&boot.app_context.db, "user1").await;
    let non_existing_user_results = Model::find_by_login(&boot.app_context.db, "user_1").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[tokio::test]
#[serial]
async fn can_find_by_pid() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let existing_user =
        Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111").await;
    let non_existing_user_results =
        Model::find_by_pid(&boot.app_context.db, "23232323-2323-2323-2323-232323232323").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[tokio::test]
#[serial]
async fn can_reset_password() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let user = Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
        .await
        .expect("Failed to find user by PID");

    assert!(
        user.verify_password("12341234"),
        "Password verification failed for original password"
    );

    let result = user
        .clone()
        .into_active_model()
        .reset_password(&boot.app_context.db, "new-password")
        .await;

    assert!(result.is_ok(), "Failed to reset password");

    let user = Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
        .await
        .expect("Failed to find user by PID after password reset");

    assert!(
        user.verify_password("new-password"),
        "Password verification failed for new password"
    );
}

#[tokio::test]
#[serial]
async fn can_find_by_claims_key() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");
    seed::<App>(&boot.app_context)
        .await
        .expect("Failed to seed database");

    let user = Model::find_by_pid(&boot.app_context.db, "11111111-1111-1111-1111-111111111111")
        .await
        .expect("Failed to find user by PID");

    let jwt = boot
        .app_context
        .config
        .auth
        .expect("Failed to fetch auth")
        .jwt
        .expect("Failed to fetch jwt settings");

    let jwt_token = user
        .generate_jwt(&jwt.secret, jwt.expiration)
        .expect("Failed to generate jwt token");

    let validate_token = loco_rs::auth::jwt::JWT::new(&jwt.secret).validate(&jwt_token);

    insta::with_settings!({
        filters => vec![
            (r"exp: ([0-9]{10}),", "exp: \"exp_in_seconds\",")
            ]
        }, {
        assert_debug_snapshot!(validate_token);
    });

    let validate_token = validate_token.expect("Failed to validate jwt");

    let user = Model::find_by_claims_key(&boot.app_context.db, &validate_token.claims.pid).await;

    assert_debug_snapshot!(user);
}
