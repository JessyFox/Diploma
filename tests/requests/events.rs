use insta::assert_debug_snapshot;
use loco_rs::testing::prelude::*;
use serial_test::serial;
use stat_api_rs::app::App;
use uuid::Uuid;

// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("auth_request");
        let _guard = settings.bind_to_scope();
    };
}

/// Тест успешной регистрации события
#[tokio::test]
#[serial]
async fn can_register_event() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let client_uuid = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let request_body = serde_json::json!({
            "event_type_id": 1,
            "client_id": client_uuid,
            "payloads": {
                "1": "/api/events",
            }
        });

        let response = request.put("/api/events").json(&request_body).await;
        assert_eq!(
            response.status_code(),
            200,
            "Event registration request should succeed"
        );

        assert_debug_snapshot!(response.text());
    })
    .await;
}

/// Тест ошибки валидации типа значения payload (422)
#[tokio::test]
#[serial]
async fn cant_register_event_with_invalid_payload_type() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let client_uuid = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let request_body = serde_json::json!({
            "event_type_id": 2,
            "client_id": client_uuid,
            "payloads": {
                "2": "a-309",
                "3": "asdasd"
            }
        });

        let response = request.put("/api/events").json(&request_body).await;
        assert_eq!(
            response.status_code(),
            422,
            "Should reject invalid payload type"
        );

        assert_debug_snapshot!(response.text());
    })
    .await;
}

/// Тест ошибки разрешённых типов полезной нагрузки (400)
#[tokio::test]
#[serial]
async fn cant_register_event_with_disallowed_payload() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let client_uuid = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let request_body = serde_json::json!({
            "event_type_id": 1,
            "client_id": client_uuid,
            "payloads": {
                "99": "some_value"
            }
        });

        let response = request.put("/api/events").json(&request_body).await;
        assert_eq!(
            response.status_code(),
            400,
            "Should reject disallowed payload types"
        );

        assert_debug_snapshot!(response.text());
    })
    .await;
}

/// Тест ошибки валидации размера полезной нагрузки (400)
#[tokio::test]
#[serial]
async fn cant_register_event_with_non_valid_payload_size() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let client_uuid = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let request_body = serde_json::json!({
            "event_type_id": 1,
            "client_id": client_uuid,
            "payloads": {
                "1": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }
        });

        let response = request.put("/api/events").json(&request_body).await;
        assert_eq!(
            response.status_code(),
            400,
            "Should reject disallowed payload types"
        );

        assert_debug_snapshot!(response.text());
    })
    .await;
}

/// Тест отсутствия клиента (404 / EntityNotFound)
#[tokio::test]
#[serial]
async fn cant_register_event_with_unknown_client() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let unknown_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let request_body = serde_json::json!({
            "event_type_id": 1,
            "client_id": unknown_uuid,
            "payloads": {
                "1": "100"
            }
        });

        let response = request.put("/api/events").json(&request_body).await;
        assert_eq!(response.status_code(), 404, "Should reject unknown client");

        assert_debug_snapshot!(response.text());
    })
    .await;
}
