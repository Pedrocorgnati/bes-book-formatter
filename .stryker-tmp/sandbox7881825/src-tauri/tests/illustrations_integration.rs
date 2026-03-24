/// Integration tests — Illustrations pipeline.
///
/// Tests cross-component interactions between ProjectRepository,
/// IllustrationRepository, and IllustrationSync.
///
/// Scenarios:
///  1. Full illustration lifecycle: pending → imported → linked (with image data)
///  2. UNIQUE constraint: duplicate placeholder_name on same project
///  3. Illustration belongs to project: isolation between projects
///  4. IllustrationSync: new placeholders become PENDING
///  5. IllustrationSync: re-sync preserves IMPORTED state (user work protected)
///  6. IllustrationSync: PENDING removed when no longer in AST
///  7. IllustrationSync: LINKED preserved when removed from AST
///  8. Multiple syncs are idempotent (no duplicates)
///  9. count_by_project reflects actual count
/// 10. Invalid state transition via update_state (VAL guard via error code)
/// 11. alt_text update persists correctly
/// 12. error state via M002 migration (state IN ('pending','imported','linked','error'))

mod common;

use bes_book_formatter_lib::{
    models::{IllustrationRef, NewIllustration},
    repositories::{IllustrationRepository, ProjectRepository},
    services::IllustrationSync,
};

// ──────────────────────────────────────────────────────────
// 1. Full lifecycle: pending → imported → linked
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_illustration_full_lifecycle() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    // CREATE (state = pending)
    let illus = repo
        .create(&NewIllustration {
            project_id: project_id.clone(),
            placeholder_name: "hero-battle".to_string(),
            description: Some("The final battle scene".to_string()),
        })
        .await
        .expect("create failed");

    assert_eq!(illus.state, "pending");
    assert_eq!(illus.placeholder_name, "hero-battle");
    assert!(illus.image_path.is_none());
    assert!(illus.validated_dpi.is_none());

    // UPDATE IMAGE (state becomes imported)
    let imported = repo
        .update_image(
            &illus.id,
            "/path/to/hero-battle.jpg",
            300,
            Some(1200),
            Some(900),
            Some("srgb"),
        )
        .await
        .expect("update_image failed");

    assert_eq!(imported.state, "imported");
    assert_eq!(imported.image_path, Some("/path/to/hero-battle.jpg".to_string()));
    assert_eq!(imported.validated_dpi, Some(300));
    assert_eq!(imported.width_px, Some(1200));
    assert_eq!(imported.height_px, Some(900));
    assert_eq!(imported.color_space, Some("srgb".to_string()));

    // TRANSITION to linked
    let linked = repo
        .update_state(&illus.id, "linked")
        .await
        .expect("update_state to linked failed");

    assert_eq!(linked.state, "linked");

    // READ back — persisted
    let found = repo
        .find_by_id(&illus.id)
        .await
        .expect("find_by_id failed")
        .expect("should exist");
    assert_eq!(found.state, "linked");
    assert_eq!(found.image_path, Some("/path/to/hero-battle.jpg".to_string()));
}

// ──────────────────────────────────────────────────────────
// 2. UNIQUE constraint: duplicate placeholder_name on same project
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_duplicate_placeholder_name_fails() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool);

    repo.create(&NewIllustration {
        project_id: project_id.clone(),
        placeholder_name: "cover".to_string(),
        description: None,
    })
    .await
    .expect("first create failed");

    let result = repo
        .create(&NewIllustration {
            project_id: project_id.clone(),
            placeholder_name: "cover".to_string(), // same name + same project
            description: Some("duplicate".to_string()),
        })
        .await;

    assert!(
        result.is_err(),
        "Expected UNIQUE constraint error for duplicate placeholder_name on same project"
    );
}

// ──────────────────────────────────────────────────────────
// 3. Illustration isolation between projects
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_illustrations_isolated_between_projects() {
    let pool = common::setup_pool().await;
    let project_a = common::make_project_named(&pool, "Project A").await;
    let project_b = common::make_project_named(&pool, "Project B").await;
    let repo = IllustrationRepository::new(pool);

    // Both projects can have an illustration with the same name
    repo.create(&NewIllustration {
        project_id: project_a.clone(),
        placeholder_name: "cover".to_string(),
        description: None,
    })
    .await
    .expect("create for project A failed");

    repo.create(&NewIllustration {
        project_id: project_b.clone(),
        placeholder_name: "cover".to_string(), // same name, different project — OK
        description: None,
    })
    .await
    .expect("create for project B failed");

    let list_a = repo.find_by_project(&project_a).await.expect("find A failed");
    let list_b = repo.find_by_project(&project_b).await.expect("find B failed");

    assert_eq!(list_a.len(), 1);
    assert_eq!(list_b.len(), 1);
    assert_ne!(list_a[0].id, list_b[0].id);
    assert_eq!(list_a[0].project_id, project_a);
    assert_eq!(list_b[0].project_id, project_b);
}

// ──────────────────────────────────────────────────────────
// 4–8. IllustrationSync cross-module scenarios
// ──────────────────────────────────────────────────────────

fn make_illus_ref(name: &str, desc: &str) -> IllustrationRef {
    IllustrationRef {
        name: name.to_string(),
        description: desc.to_string(),
        context: String::new(),
        line_number: 1,
        file_path: "/tmp/ch.md".to_string(),
    }
}

#[tokio::test]
async fn test_sync_creates_new_pending_illustrations() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;

    let ast_refs = vec![
        make_illus_ref("cover", "Book cover"),
        make_illus_ref("chapter-1-map", "Map of the kingdom"),
        make_illus_ref("epilogue-portrait", "Author portrait"),
    ];

    IllustrationSync::sync(&pool, &project_id, &ast_refs)
        .await
        .expect("sync failed");

    let repo = IllustrationRepository::new(pool);
    let items = repo.find_by_project(&project_id).await.expect("find failed");

    assert_eq!(items.len(), 3, "Expected 3 illustrations after sync");
    for item in &items {
        assert_eq!(item.state, "pending", "New sync items must be PENDING");
    }
}

#[tokio::test]
async fn test_sync_preserves_imported_state_on_re_sync() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    // First sync: creates pending
    IllustrationSync::sync(&pool, &project_id, &[make_illus_ref("cover", "Cover")])
        .await
        .expect("first sync failed");

    // Mark as imported (user linked an image)
    let illus = repo
        .find_by_project(&project_id)
        .await
        .expect("find failed")[0]
        .clone();
    repo.update_state(&illus.id, "imported")
        .await
        .expect("update state failed");

    // Second sync — 'cover' still in AST: must NOT be overwritten
    IllustrationSync::sync(&pool, &project_id, &[make_illus_ref("cover", "Cover")])
        .await
        .expect("second sync failed");

    let items = repo.find_by_project(&project_id).await.expect("find after re-sync failed");
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].state, "imported",
        "IMPORTED state must be preserved on re-sync"
    );
}

#[tokio::test]
async fn test_sync_removes_pending_when_removed_from_ast() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    // Create two illustrations: one will be kept, one will be removed from AST
    IllustrationSync::sync(
        &pool,
        &project_id,
        &[
            make_illus_ref("keep-this", "Keeper"),
            make_illus_ref("remove-this", "Will be deleted"),
        ],
    )
    .await
    .expect("initial sync failed");

    // Re-sync with only one item — 'remove-this' was PENDING → must be deleted
    IllustrationSync::sync(&pool, &project_id, &[make_illus_ref("keep-this", "Keeper")])
        .await
        .expect("second sync failed");

    let items = repo.find_by_project(&project_id).await.expect("find failed");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].placeholder_name, "keep-this");
}

#[tokio::test]
async fn test_sync_preserves_linked_even_when_removed_from_ast() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    // Create and link an illustration
    IllustrationSync::sync(&pool, &project_id, &[make_illus_ref("linked-fig", "Linked")])
        .await
        .expect("initial sync failed");
    let illus = repo.find_by_project(&project_id).await.expect("find failed")[0].clone();
    repo.update_state(&illus.id, "linked").await.expect("link failed");

    // Sync with empty AST — LINKED must be preserved (user work)
    IllustrationSync::sync(&pool, &project_id, &[])
        .await
        .expect("sync with empty AST failed");

    let items = repo.find_by_project(&project_id).await.expect("find after empty sync failed");
    assert_eq!(items.len(), 1, "LINKED illustration must survive empty AST sync");
    assert_eq!(items[0].state, "linked");
}

#[tokio::test]
async fn test_sync_is_idempotent() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;

    let refs = vec![
        make_illus_ref("fig-a", "A"),
        make_illus_ref("fig-b", "B"),
    ];

    // Run sync 3 times — result must be the same
    for _ in 0..3 {
        IllustrationSync::sync(&pool, &project_id, &refs)
            .await
            .expect("sync failed");
    }

    let repo = IllustrationRepository::new(pool);
    let items = repo.find_by_project(&project_id).await.expect("find failed");
    assert_eq!(
        items.len(),
        2,
        "Idempotent sync must not create duplicate illustrations"
    );
}

// ──────────────────────────────────────────────────────────
// 9. count_by_project
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_count_by_project() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    assert_eq!(repo.count_by_project(&project_id).await.unwrap(), 0);

    for i in 0..4 {
        repo.create(&NewIllustration {
            project_id: project_id.clone(),
            placeholder_name: format!("fig-{}", i),
            description: None,
        })
        .await
        .expect("create failed");
    }

    assert_eq!(repo.count_by_project(&project_id).await.unwrap(), 4);
}

// ──────────────────────────────────────────────────────────
// 10. Error state (from M002 migration)
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_error_state_is_valid_after_m002_migration() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool);

    let illus = repo
        .create(&NewIllustration {
            project_id: project_id.clone(),
            placeholder_name: "error-fig".to_string(),
            description: None,
        })
        .await
        .expect("create failed");

    // 'error' state is valid after M002 migration
    let result = repo.update_state(&illus.id, "error").await;
    assert!(
        result.is_ok(),
        "Expected 'error' state to be valid after M002 migration, got: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().state, "error");
}

// ──────────────────────────────────────────────────────────
// 11. alt_text update
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_alt_text_update_persists() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool);

    let illus = repo
        .create(&NewIllustration {
            project_id,
            placeholder_name: "portrait".to_string(),
            description: None,
        })
        .await
        .expect("create failed");

    assert!(illus.alt_text.is_none());

    let updated = repo
        .update_alt_text(&illus.id, "Retrato do autor na juventude")
        .await
        .expect("update_alt_text failed");

    assert_eq!(
        updated.alt_text,
        Some("Retrato do autor na juventude".to_string())
    );

    // Verify persisted in DB
    let found = repo
        .find_by_id(&illus.id)
        .await
        .expect("find failed")
        .expect("should exist");
    assert_eq!(
        found.alt_text,
        Some("Retrato do autor na juventude".to_string())
    );
}

// ──────────────────────────────────────────────────────────
// 12. find_by_state filter
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_find_by_state_filters_correctly() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = IllustrationRepository::new(pool.clone());

    // Create 3 illustrations
    let a = repo.create(&NewIllustration { project_id: project_id.clone(), placeholder_name: "fig-a".to_string(), description: None }).await.unwrap();
    let b = repo.create(&NewIllustration { project_id: project_id.clone(), placeholder_name: "fig-b".to_string(), description: None }).await.unwrap();
    let _c = repo.create(&NewIllustration { project_id: project_id.clone(), placeholder_name: "fig-c".to_string(), description: None }).await.unwrap();

    // Transition a → imported, b → linked
    repo.update_state(&a.id, "imported").await.unwrap();
    repo.update_state(&b.id, "linked").await.unwrap();

    let pending = repo.find_by_state(&project_id, "pending").await.unwrap();
    let imported = repo.find_by_state(&project_id, "imported").await.unwrap();
    let linked = repo.find_by_state(&project_id, "linked").await.unwrap();

    assert_eq!(pending.len(), 1, "Expected 1 pending");
    assert_eq!(imported.len(), 1, "Expected 1 imported");
    assert_eq!(linked.len(), 1, "Expected 1 linked");

    assert_eq!(pending[0].placeholder_name, "fig-c");
    assert_eq!(imported[0].placeholder_name, "fig-a");
    assert_eq!(linked[0].placeholder_name, "fig-b");
}
