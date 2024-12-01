use actix_web::web;
use mongodb::{bson::{doc, to_bson}, error::Error, results::{DeleteResult, InsertOneResult, UpdateResult}, Client, Cursor};
use crate::corporate_modules::models::training::Training;


pub async fn get_training(client: web::Data<Client>) -> Result<Cursor<Training>, Error> {
    let corporate_training_collection = client.database("iykra-online").collection::<Training>("training");
    corporate_training_collection.find(doc! {}).await
}

pub async fn update_training(client: web::Data<Client>, data_param: web::Json<Training>) -> Result<UpdateResult, Error> {
    let corporate_training_collection = client.database("iykra-online").collection::<Training>("training");
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

    if let Some(contents) = &data_param.contents {
        let content_bson = to_bson(contents).expect("Failed to serialized contents");
        to_be_updated.insert("contents", content_bson);
    }

    if let Some(use_case_title) = &data_param.use_case_title {
        to_be_updated.insert("use_case_title", use_case_title);
    }

    if let Some(use_cases) = &data_param.use_cases {
        let use_cases_bson = to_bson(use_cases).expect("Failed to serialized use_cases");
        to_be_updated.insert("use_cases", use_cases_bson);
    }

    if let Some(experiences) = &data_param.experiences {
        let experiences_bson = to_bson(experiences).expect("Failed to serialized use_cases");
        to_be_updated.insert("experiences", experiences_bson);
    }

    if let Some(unique_selling_point) = &data_param.unique_selling_point {
        let unique_selling_point_bson = to_bson(unique_selling_point).expect("Failed to serialized use_cases");
        to_be_updated.insert("unique_selling_point", unique_selling_point_bson);
    }

    let update = doc! {
        "$set": to_be_updated
    };

    corporate_training_collection.update_one(query, update).await
}

pub async fn create_training(client: web::Data<Client>, created_data: web::Json<Training>) -> Result<InsertOneResult, Error> {
    let corporate_training_collection = client.database("iykra-online").collection::<Training>("training");
    corporate_training_collection.insert_one(created_data.into_inner()).await
}

pub async fn delete_training(client: web::Data<Client>, deleted_data: web::Json<Training>) -> Result<DeleteResult, Error> {
    let corporate_training_collection = client.database("iykra-online").collection::<Training>("training");
    corporate_training_collection.delete_one(doc! {"_id": deleted_data._id}).await
}