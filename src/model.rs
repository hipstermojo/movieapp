#[derive(Debug, Serialize, Deserialize)]
pub struct APIMovieData {
    pub backdrop_path: String,
    pub genre_ids: Vec<u32>,
    pub id: u32,
    pub original_language: String,
    pub title: String,
    pub overview: String,
    pub poster_path: String,
    pub release_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub results: Vec<APIMovieData>,
}

pub type APIKey = String;