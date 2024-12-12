use serde_json::json;

mod utils;

use utils::util::get_result;

const QUERY: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      english
      native
    }
  }
}
";

const PAGED_QUERY: &str = "
query ($id: Int, $page: Int, $perPage: Int, $search: String) {
    Page (page: $page, perPage: $perPage) {
        pageInfo {
            currentPage
            hasNextPage
            perPage
        }
        media (id: $id, search: $search) {
            id
            title {
                english
                native
            }
        }
    }
}
";

const CUSTOM_QUERY: &str = "
query ($page: Int, $perPage: Int, $type: MediaType) {
    Page (page: $page, perPage: $perPage) {
        pageInfo {
            currentPage
            hasNextPage
            perPage
        }
        media (type: $type, sort: [SCORE_DESC, POPULARITY_DESC]) {
            id
            title {
                english
                native
            }
            meanScore
            popularity
            episodes
            status
        }
    }
}
";

#[tokio::main]
async fn main() {
    let mut json;
    let mut result: serde_json::Value;

    json = json!({"query": QUERY, "variables": {"id": 15125}});
    result = get_result(json.to_string()).await;
    println!("{:#}", result);

    json = json!({"query": PAGED_QUERY, "variables": {
        "search": "Fate/Zero",
        "page": 1,
        "perPage": 3,
    }});
    result = get_result(json.to_string()).await;
    println!("{:#}", result);

    json = json!({
        "query": CUSTOM_QUERY,
        "variables": {
            "page": 1,
            "perPage": 10,
            "type": "ANIME"
        }
    });
    result = get_result(json.to_string()).await;
    println!("Top anime result:\n{:#}\n", result);

    // Custom query for top manga
    json = json!({
        "query": CUSTOM_QUERY,
        "variables": {
            "page": 1,
            "perPage": 10,
            "type": "MANGA"
        }
    });
    result = get_result(json.to_string()).await;
    println!("Top manga result:\n{:#}", result);
}
