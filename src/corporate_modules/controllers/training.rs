use std::vec;

use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use crate::corporate_modules::{models::training::Training, services::training::{get_training, create_training, update_training, delete_training}};
use futures::stream::StreamExt;

#[derive(serde::Serialize)]
struct ResponseTraining<T> {
    message: String,
    data: T,
}

pub async fn index(client: web::Data<Client>) -> impl Responder  {
    match get_training(client).await {
        Ok(mut cursor) => {
            let mut resp = ResponseTraining {
                message: String::from("Success get training"),
                data: vec![],
            };

            // Process each document in the cursor
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(item) => {
                        print!("item {:?}", item);
                        resp.data.push(item);
                    }, // Successfully parsed document
                    Err(err) => {
                        // Handle any errors while iterating
                        resp.message = String::from("Failed get training");
                    }
                }
            }

            // Build and return the response
            
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Service error: {}", e)),
    }
}

pub async fn update(client: web::Data<Client>, updated_data: web::Json<Training>) -> impl Responder {
    match &updated_data._id {
        Some(id) => {
            match update_training(client, updated_data).await {
                Ok(update_res) => {
                    let resp = ResponseTraining {
                        message: String::from("Successfully updated about information"),
                        data: Some(update_res),
                    };
                    HttpResponse::Ok().json(resp)
                },
                Err(e) => {
                    let resp: ResponseTraining<Vec<String>> = ResponseTraining {
                        message: e.to_string(),
                        data: vec![],
                    };
                    HttpResponse::Ok().json(resp)
                },
            }
        },
        None => {
            let resp: ResponseTraining<Vec<String>> = ResponseTraining {
                message: String::from("_id is required"),
                data: vec![],
            };
            HttpResponse::Ok().json(resp)
        }
    }
}

pub async fn create(client: web::Data<Client>, created_data: web::Json<Training>) -> impl Responder {
    let mut resp: ResponseTraining<String> = ResponseTraining {
        message: String::new(),
        data: String::new()
    };

    match &created_data.title {
        Some(title) => if title.is_empty() { resp.message = String::from("Title cannot be empty"); },
        None => { resp.message = String::from("Title cannot be empty"); }
    }

    match &created_data.caption {
        Some(caption) => if caption.is_empty() { resp.message = String::from("Caption cannot be empty"); },
        None => { resp.message = String::from("Caption cannot be empty"); }
    }

    match &created_data.banner {
        Some(banner) => if banner.is_empty() { resp.message = String::from("Banner cannot be empty"); },
        None => { resp.message = String::from("Banner cannot be empty"); }
    }

    match &created_data.content_title {
        Some(content_title) => if content_title.is_empty() { resp.message = String::from("Content Title cannot be empty"); },
        None => { resp.message = String::from("Content Title cannot be empty"); }
    }

    match &created_data.contents {
        Some(contents) => if contents.is_empty() { resp.message = String::from("Contents cannot be empty"); },
        None => { resp.message = String::from("Contents cannot be empty"); }
    }

    match &created_data.use_case_title {
        Some(use_case_title) => if use_case_title.is_empty() { resp.message = String::from("Use case Title cannot be empty"); },
        None => { resp.message = String::from("Use case Title cannot be empty"); }
    }

    match &created_data.use_cases {
        Some(use_cases) => if use_cases.is_empty() { resp.message = String::from("Use cases cannot be empty"); },
        None => { resp.message = String::from("Use cases cannot be empty"); }
    }

    match &created_data.experiences {
        Some(experiences) => if experiences.is_empty() { resp.message = String::from("Experiences cannot be empty"); },
        None => { resp.message = String::from("Experiences cannot be empty"); }
    }

    match &created_data.unique_selling_point {
        Some(unique_selling_point) => if unique_selling_point.is_empty() { resp.message = String::from("Unique selling point cannot be empty"); },
        None => { resp.message = String::from("Unique selling point cannot be empty"); }
    }

    match create_training(client, created_data).await {
        Ok(created_result) => {
            resp.message = String::from("Training is created successfully");
            resp.data = created_result.inserted_id.to_string();
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}

pub async fn delete(client: web::Data<Client>, deleted_data: web::Json<Training>) -> impl Responder {
    let mut resp: ResponseTraining<u64> = ResponseTraining {
        message: String::new(),
        data: 0
    };

    match delete_training(client, deleted_data).await {
        Ok(result) => {
            resp.message = String::from("Training deleted successfully");
            
            if result.deleted_count == 0 {
                resp.message = String::from("Failed to delete training");
            }
            
            resp.data = result.deleted_count;
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}