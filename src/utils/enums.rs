use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum MediaType {
    ANIME,
    MANGA,
}

#[derive(Debug, Serialize)]
pub enum MediaStatus {
    FINISHED,
    RELEASING,
    NOT_YET_RELEASED,
    CANCELLED,
    HIATUS,
}
