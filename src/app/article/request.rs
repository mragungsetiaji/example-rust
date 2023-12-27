use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub article: ArticleRequest,
}

#[derive(Deserialize, Serialize)]
pub struct ArticleContent {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Option<Vec<String>>,
}