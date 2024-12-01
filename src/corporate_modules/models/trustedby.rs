use mongodb::bson::oid::ObjectId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrustedBy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub title: Option<String>,
    pub banner: Option<String>,
}