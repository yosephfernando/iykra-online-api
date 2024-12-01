use std::vec;

use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use crate::about_modules::{models::about::About, services::about::{get_about, update_about, create_about, delete_about}};
use futures::stream::StreamExt;

#[derive(serde::Serialize)]
struct ResponseAbout<T> {
    message: String,
    data: T,
}

pub async fn index(client: web::Data<Client>) -> impl Responder  {
    match get_about(client).await {
        Ok(mut cursor) => {
            let mut resp = ResponseAbout {
                message:  String::from("Success get about"),
                data: vec![],
            };

            // Process each document in the cursor
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(item) => {
                        resp.data.push(item);
                    }, // Successfully parsed document
                    Err(err) => {
                        // Handle any errors while iterating
                        resp.message = String::from("Failed get about");
                    }
                }
            }

            // Build and return the response
            
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Service error: {}", e)),
    }
}

pub async fn update(client: web::Data<Client>, updated_data: web::Json<About>) -> impl Responder {
    match &updated_data._id {
        Some(id) => {
            match update_about(client, updated_data).await {
                Ok(update_res) => {
                    let resp = ResponseAbout {
                        message: String::from("Successfully updated about information"),
                        data: Some(update_res),
                    };
                    HttpResponse::Ok().json(resp)
                },
                Err(e) => {
                    let resp: ResponseAbout<Vec<String>> = ResponseAbout {
                        message: e.to_string(),
                        data: vec![],
                    };
                    HttpResponse::Ok().json(resp)
                },
            }
        },
        None => {
            let resp: ResponseAbout<Vec<String>> = ResponseAbout {
                message: String::from("_id is required"),
                data: vec![],
            };
            HttpResponse::Ok().json(resp)
        }
    }
}

pub async fn create(client: web::Data<Client>, created_data: web::Json<About>) -> impl Responder {
    let mut resp: ResponseAbout<String> = ResponseAbout {
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

    match &created_data.content_caption {
        Some(content_caption) => if content_caption.is_empty() { resp.message = String::from("Content Caption cannot be empty"); },
        None => { resp.message = String::from("Content Caption cannot be empty"); }
    }

    match &created_data.video_source {
        Some(video_source) => if video_source.is_empty() { resp.message = String::from("Video source cannot be empty"); },
        None => { resp.message = String::from("Video source cannot be empty"); }
    }

    match &created_data.our_values {
        Some(our_values) => if our_values.len() == 0 { resp.message = String::from("Our values cannot be empty"); },
        None => { resp.message = String::from("Our values cannot be empty"); }
    }

    match create_about(client, created_data).await {
        Ok(created_result) => {
            resp.message = String::from("About is created successfully");
            resp.data = created_result.inserted_id.to_string();
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}

pub async fn delete(client: web::Data<Client>, deleted_data: web::Json<About>) -> impl Responder {
    let mut resp: ResponseAbout<u64> = ResponseAbout {
        message: String::new(),
        data: 0
    };

    match delete_about(client, deleted_data).await {
        Ok(result) => {
            resp.message = String::from("About deleted successfully");
            
            if result.deleted_count == 0 {
                resp.message = String::from("Failed to delete about");
            }
            
            resp.data = result.deleted_count;
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}