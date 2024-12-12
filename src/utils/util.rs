use reqwest::Client;

const QUERY: &str = "
query ($page: Int, $perPage: Int, $type: MediaType, $status: MediaStatus) {
    Page (page: $page, perPage: $perPage) {
        pageInfo {
            currentPage
            hasNextPage
            perPage
        }
        media (
            type: $type, 
            sort: [SCORE_DESC, POPULARITY_DESC],
            status: $status
        ) {
            id
            title {
                english
                native
                romaji
            }
            meanScore
            popularity
            episodes
            chapters
            status
            description
            coverImage {
                large
            }
            startDate {
                year
                month
                day
            }
        }
    }
}
";

pub async fn get_result(json: String) -> serde_json::Value {
    let client = Client::new();
    let response = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    serde_json::from_str(&response.unwrap()).unwrap()
}
