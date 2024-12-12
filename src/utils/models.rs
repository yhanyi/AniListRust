#[allow(dead_code)]
pub const TEST_QUERY_1: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      english
      romaji
      native
    }
  }
}
";

#[allow(dead_code)]
pub const TEST_QUERY_2: &str = "
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
                romaji
            }
        }
    }
}
";

pub const QUERY: &str = "
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
