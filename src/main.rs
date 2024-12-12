mod utils;

use utils::enums::{MediaStatus, MediaType};
use utils::util::fetch;
use utils::util::QueryOptions;

#[tokio::main]
async fn main() {
    let anime_options = QueryOptions {
        media_type: MediaType::Anime,
        page: 1,
        per_page: 10,
        status: Some(MediaStatus::Finished),
    };

    match fetch(anime_options).await {
        Ok(result) => println!("Top Finished Anime:\n{:#}\n", result),
        Err(e) => eprintln!("Error fetching anime: {}", e),
    }

    let manga_options = QueryOptions {
        media_type: MediaType::Manga,
        page: 1,
        per_page: 5,
        status: Some(MediaStatus::Releasing),
    };

    match fetch(manga_options).await {
        Ok(result) => println!("Currently Releasing Manga:\n{:#}\n", result),
        Err(e) => eprintln!("Error fetching manga: {}", e),
    }
}
