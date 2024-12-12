use anilist_rust::utils::models::{TEST_QUERY_1, TEST_QUERY_2};
use anilist_rust::utils::util::get_result;
use serde_json::json;

const EXPECTED_RESULT_1: &str = r#"{
    "data": {
        "Media": {
            "id": 15125,
            "title": {
                "romaji": "Teekyuu",
                "english": "Teekyuu",
                "native": "てーきゅう"
            }
        }
    }
}"#;

const EXPECTED_RESULT_2: &str = r#"{
    "data": {
        "Page": {
            "pageInfo": {
                "currentPage": 1,
                "hasNextPage": true,
                "perPage": 3
            },
            "media": [
                {
                    "id": 55191,
                    "title": {
                        "romaji": "Fate/Zero"
                    }
                },
                {
                    "id": 10087,
                    "title": {
                        "romaji": "Fate/Zero"
                    }
                },
                {
                    "id": 33649,
                    "title": {
                        "romaji": "Fate/Zero"
                    }
                }
            ]
        }
    }
}"#;

#[tokio::test]
async fn test_basic_anime_query() {
    let json = json!({
        "query": TEST_QUERY_1,
        "variables": {
            "id": 15125
        }
    });

    let result = get_result(json.to_string()).await;
    let expected: serde_json::Value =
        serde_json::from_str(EXPECTED_RESULT_1).expect("Expected result should be valid JSON");

    assert_eq!(result, expected, "Response should match expected JSON");
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
    let expected: serde_json::Value =
        serde_json::from_str(EXPECTED_RESULT_2).expect("Expected result should be valid JSON");

    assert_eq!(result, expected, "Response should match expected JSON");
}
