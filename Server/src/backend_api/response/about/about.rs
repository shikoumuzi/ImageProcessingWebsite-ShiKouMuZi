// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

// extern crate serde_derive;

use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    /// 下载链接
    #[serde(rename = "download_url")]
    pub download_url: Vec<DownloadUrl>,

    /// 介绍
    #[serde(rename = "introduction")]
    pub introduction: String,

    /// 官方链接
    #[serde(rename = "offical_url")]
    pub offical_url: Vec<OfficalUrl>,

    /// 推荐文章链接
    #[serde(rename = "recommended_article_url")]
    pub recommended_article_url: Vec<RecommendedArticleUrl>,

    /// 状态码
    #[serde(rename = "status")]
    pub status: u8,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadUrl {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct OfficalUrl {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecommendedArticleUrl {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,
}

impl Response {
    pub fn new(download_url: Vec<DownloadUrl>, introduction: String, offical_url: Vec<OfficalUrl>, recommended_article_url: Vec<RecommendedArticleUrl>, status: u8) -> Self {
        Self { download_url, introduction, offical_url, recommended_article_url, status }
    }
}

impl DownloadUrl {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

impl OfficalUrl {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

impl RecommendedArticleUrl {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}