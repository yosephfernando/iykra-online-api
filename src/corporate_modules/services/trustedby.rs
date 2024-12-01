use actix_web::web;
use mongodb::{bson::doc, error::Error, results::{DeleteResult, InsertOneResult, UpdateResult}, Client, Cursor};
use crate::corporate_modules::models::trustedby::TrustedBy;


pub async fn get_trusted_by(client: web::Data<Client>) -> Result<Cursor<TrustedBy>, Error> {
    let trusted_by_collection = client.database("iykra-online").collection::<TrustedBy>("trusted_by");
    trusted_by_collection.find(doc! {}).await
}

pub async fn update_trusted_by(client: web::Data<Client>, data_param: web::Json<TrustedBy>) -> Result<UpdateResult, Error> {
    let trusted_by_collection = client.database("iykra-online").collection::<TrustedBy>("trusted_by");
    let id = &data_param._id;
    let query   = doc! {
        "_id": id
    };

    let mut to_be_updated = doc! {};

    if let Some(title) = &data_param.title {
        to_be_updated.insert("title", title);
    }

    if let Some(banner) = &data_param.banner {
        to_be_updated.insert("banner", banner);
    }

    let update = doc! {
        "$set": to_be_updated
    };

    trusted_by_collection.update_one(query, update).await
}

pub async fn create_trusted_by(client: web::Data<Client>, created_data: web::Json<TrustedBy>) -> Result<InsertOneResult, Error> {
    let trusted_by_collection = client.database("iykra-online").collection::<TrustedBy>("trusted_by");
    trusted_by_collection.insert_one(created_data.into_inner()).await
}

pub async fn delete_trusted_by(client: web::Data<Client>, deleted_data: web::Json<TrustedBy>) -> Result<DeleteResult, Error> {
    let trusted_by_collection = client.database("iykra-online").collection::<TrustedBy>("trusted_by");
    trusted_by_collection.delete_one(doc! {"_id": deleted_data._id}).await
}