/// Integration tests — User Preferences.
///
/// Covers M001 seed data, valid/invalid key enforcement (PREF_001),
/// and cross-request persistence of preference values.
///
/// Scenarios:
///  1. M001 seeds three defaults: theme=light, ui_language=pt-BR, analytics_opt_in=false
///  2. Set valid key → persisted → readable
///  3. Set invalid key → PREF_001 error (not persisted)
///  4. Overwrite existing preference → latest value wins
///  5. get() for non-existent key → None (not error)
///  6. get_all() returns all preferences including user-set ones
///  7. All three valid keys accepted: theme, ui_language, analytics_opt_in

mod common;

use bes_book_formatter_lib::repositories::PreferenceRepository;

// ──────────────────────────────────────────────────────────
// 1. M001 default seeds
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_migration_seeds_default_preferences() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    let theme = repo.get("theme").await.expect("get theme failed");
    let lang = repo.get("ui_language").await.expect("get ui_language failed");
    let analytics = repo.get("analytics_opt_in").await.expect("get analytics failed");

    assert_eq!(theme, Some("light".to_string()), "Default theme must be 'light'");
    assert_eq!(lang, Some("pt-BR".to_string()), "Default ui_language must be 'pt-BR'");
    assert_eq!(
        analytics,
        Some("false".to_string()),
        "Default analytics_opt_in must be 'false'"
    );
}

// ──────────────────────────────────────────────────────────
// 2. Set valid key and read back
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_set_valid_key_persists() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set("theme", "dark").await.expect("set theme failed");
    let value = repo.get("theme").await.expect("get theme failed");

    assert_eq!(value, Some("dark".to_string()));
}

#[tokio::test]
async fn test_set_ui_language_valid() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set("ui_language", "en-US").await.expect("set ui_language failed");
    let value = repo.get("ui_language").await.expect("get failed");

    assert_eq!(value, Some("en-US".to_string()));
}

#[tokio::test]
async fn test_set_analytics_opt_in_true() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set("analytics_opt_in", "true")
        .await
        .expect("set analytics_opt_in failed");
    let value = repo.get("analytics_opt_in").await.expect("get failed");

    assert_eq!(value, Some("true".to_string()));
}

// ──────────────────────────────────────────────────────────
// 3. Set invalid key → PREF_001
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_set_invalid_key_returns_pref_001() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool.clone());

    for invalid_key in &["unknown_pref", "THEME", "sidebar_width", ""] {
        let result = repo.set(invalid_key, "any_value").await;
        assert!(
            result.is_err(),
            "Expected PREF_001 for key '{}', but got Ok",
            invalid_key
        );
        let err = result.unwrap_err();
        assert_eq!(
            err.code, "PREF_001",
            "Expected PREF_001 for key '{}', got code: {}",
            invalid_key, err.code
        );
    }
}

#[tokio::test]
async fn test_invalid_key_not_persisted() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    let _ = repo.set("hacker_key", "injected").await;

    // Key was never inserted
    let value = repo.get("hacker_key").await.expect("get failed");
    assert!(
        value.is_none(),
        "Invalid key must not be persisted, but got: {:?}",
        value
    );
}

// ──────────────────────────────────────────────────────────
// 4. Overwrite: latest value wins
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_overwrite_preference_latest_value_wins() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set("theme", "dark").await.expect("set dark failed");
    repo.set("theme", "light").await.expect("set light failed");
    repo.set("theme", "dark").await.expect("set dark again failed");

    let value = repo.get("theme").await.expect("get failed");
    assert_eq!(value, Some("dark".to_string()), "Last written value must win");
}

// ──────────────────────────────────────────────────────────
// 5. get() for non-existent key → None
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_nonexistent_key_returns_none() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    // 'sidebar_open' was never set
    let value = repo.get("sidebar_open").await.expect("query should not fail");
    assert!(value.is_none());
}

// ──────────────────────────────────────────────────────────
// 6. get_all() includes user-set preferences
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_all_includes_user_set_preferences() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set("theme", "dark").await.expect("set failed");
    repo.set("ui_language", "it-IT").await.expect("set failed");

    let all = repo.get_all().await.expect("get_all failed");

    assert_eq!(all.get("theme"), Some(&"dark".to_string()));
    assert_eq!(all.get("ui_language"), Some(&"it-IT".to_string()));
    // Default analytics still present
    assert_eq!(
        all.get("analytics_opt_in"),
        Some(&"false".to_string()),
        "Default seed must still be present"
    );
}

// ──────────────────────────────────────────────────────────
// 7. set_raw bypasses key validation (preview state storage)
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_set_raw_persists_arbitrary_key() {
    let pool = common::setup_pool().await;
    let repo = PreferenceRepository::new(pool);

    repo.set_raw("preview_zoom_level", "125")
        .await
        .expect("set_raw failed");

    let value = repo.get("preview_zoom_level").await.expect("get failed");
    assert_eq!(
        value,
        Some("125".to_string()),
        "set_raw must persist arbitrary keys without validation"
    );
}
