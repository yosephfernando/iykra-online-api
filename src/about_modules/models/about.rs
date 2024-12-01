use mongodb::bson::oid::ObjectId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct About {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub banner: Option<String>,
    pub content_title: Option<String>,
    pub content_caption: Option<String>,
    pub video_source: Option<String>,
    pub our_values: Option<Vec<String>>,
}