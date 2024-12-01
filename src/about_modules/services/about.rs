use actix_web::web;
use mongodb::{bson::doc, error::Error, results::{DeleteResult, InsertOneResult, UpdateResult}, Client, Cursor};
use crate::about_modules::models::about::About;


pub async fn get_about(client: web::Data<Client>) -> Result<Cursor<About>, Error> {
    let about_collection = client.database("iykra-online").collection::<About>("about");
    about_collection.find(doc! {}).await
}

pub async fn update_about(client: web::Data<Client>, data_param: web::Json<About>) -> Result<UpdateResult, Error> {
    let about_collection = client.database("iykra-online").collection::<About>("about");
    let id = &data_param._id;
    let query   = doc! {
        "_id": id
    };

    let mut to_be_updated = doc! {};

    if let Some(title) = &data_param.title {
        to_be_updated.insert("title", title);
    }

    if let Some(caption) = &data_param.caption {
        to_be_updated.insert("caption", caption);
    }

    if let Some(banner) = &data_param.banner {
        to_be_updated.insert("banner", banner);
    }

    if let Some(content_title) = &data_param.content_title {
        to_be_updated.insert("content_title", content_title);
    }

    if let Some(content_caption) = &data_param.content_caption {
        to_be_updated.insert("content_caption", content_caption);
    }

    if let Some(video_source) = &data_param.video_source {
        to_be_updated.insert("video_source", video_source);
    }

    if let Some(our_values) = &data_param.our_values {
        to_be_updated.insert("our_values", our_values);
    }

    let update = doc! {
        "$set": to_be_updated
    };

    about_collection.update_one(query, update).await
}

pub async fn create_about(client: web::Data<Client>, created_data: web::Json<About>) -> Result<InsertOneResult, Error> {
    let about_collection = client.database("iykra-online").collection::<About>("about");
    about_collection.insert_one(created_data.into_inner()).await
}

pub async fn delete_about(client: web::Data<Client>, deleted_data: web::Json<About>) -> Result<DeleteResult, Error> {
    let about_collection = client.database("iykra-online").collection::<About>("about");
    about_collection.delete_one(doc! {"_id": deleted_data._id}).await
}