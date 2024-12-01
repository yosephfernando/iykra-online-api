use mongodb::bson::oid::ObjectId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Training {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub banner: Option<String>,
    pub content_title: Option<String>,
    pub contents: Option<Vec<ContentItem>>,
    pub use_case_title: Option<String>,
    pub use_cases: Option<Vec<UsecaseItem>>,
    pub experiences: Option<Vec<ExperiencesItem>>,
    pub unique_selling_point: Option<Vec<UniqueSellingPointItem>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ContentItem {
    pub title: Option<String>,
    pub caption: Option<String>,
    pub image_thumb: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsecaseItem {
    pub title: Option<String>,
    pub caption: Option<String>,
    pub image_thumb: Option<String>,
    pub purpose: Option<String>,
    pub benefits: Option<Vec<String>>,
    pub results: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExperiencesItem {
    pub category: Option<String>,
    pub image_thumb: Option<String>,
    pub products: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UniqueSellingPointItem {
    pub title: Option<String>,
    pub caption: Option<String>,
}