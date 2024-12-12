mod utils;

use utils::enums::{MediaStatus, MediaType};
use utils::util::fetch;
use utils::util::QueryOptions;

#[tokio::main]
async fn main() {
    // let mut json;
    // let mut result: serde_json::Value;

    // json = json!({"query": TEST_QUERY_1, "variables": {"id": 15125}});
    // result = get_result(json.to_string()).await;
    // println!("{:#}", result);

    // json = json!({"query": TEST_QUERY_2, "variables": {
    //     "search": "Fate/Zero",
    //     "page": 1,
    //     "perPage": 3,
    // }});
    // result = get_result(json.to_string()).await;
    // println!("{:#}", result);

    let anime_options = QueryOptions {
        media_type: MediaType::ANIME,
        page: 1,
        per_page: 10,
        status: Some(MediaStatus::FINISHED),
    };

    match fetch(anime_options).await {
        Ok(result) => println!("Top Finished Anime:\n{:#}\n", result),
        Err(e) => eprintln!("Error fetching anime: {}", e),
    }

    let manga_options = QueryOptions {
        media_type: MediaType::MANGA,
        page: 1,
        per_page: 5,
        status: Some(MediaStatus::RELEASING),
    };

    match fetch(manga_options).await {
        Ok(result) => println!("Currently Releasing Manga:\n{:#}\n", result),
        Err(e) => eprintln!("Error fetching manga: {}", e),
    }
}
