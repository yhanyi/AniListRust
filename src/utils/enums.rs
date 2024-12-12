use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MediaType {
    Anime,
    Manga,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
}
