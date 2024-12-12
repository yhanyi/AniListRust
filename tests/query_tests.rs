use anilist_rust::utils::models::{TEST_QUERY_1, TEST_QUERY_2};
use anilist_rust::utils::util::get_result;
use serde_json::json;

#[tokio::test]
async fn test_basic_anime_query() {
    let json = json!({
        "query": TEST_QUERY_1,
        "variables": {
            "id": 15125
        }
    });

    let result = get_result(json.to_string()).await;

    assert!(
        result.get("data").is_some(),
        "Response should contain data field"
    );

    let media = result["data"]["Media"]
        .as_object()
        .expect("Media should be an object");
    assert!(media.contains_key("id"), "Media should contain an id");
    assert!(media.contains_key("title"), "Media should contain a title");
}

#[tokio::test]
async fn test_paged_search_query() {
    let json = json!({
        "query": TEST_QUERY_2,
        "variables": {
            "search": "Fate/Zero",
            "page": 1,
            "perPage": 3,
        }
    });

    let result = get_result(json.to_string()).await;

    assert!(
        result.get("data").is_some(),
        "Response should contain data field"
    );

    let page = result["data"]["Page"]
        .as_object()
        .expect("Page should be an object");
    assert!(page.contains_key("pageInfo"), "Should contain pageInfo");
    assert!(page.contains_key("media"), "Should contain media");

    let media = page["media"].as_array().expect("Media should be an array");
    assert!(media.len() <= 3, "Should return at most 3 results");

    // Test first media entry has required fields
    if let Some(first_entry) = media.first() {
        let first_entry = first_entry
            .as_object()
            .expect("Media entry should be an object");
        assert!(
            first_entry.contains_key("id"),
            "Media entry should contain an id"
        );
        assert!(
            first_entry.contains_key("title"),
            "Media entry should contain a title"
        );
    }
}
