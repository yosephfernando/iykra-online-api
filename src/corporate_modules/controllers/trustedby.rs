use std::vec;

use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use crate::corporate_modules::{models::trustedby::TrustedBy, services::trustedby::{get_trusted_by, update_trusted_by, create_trusted_by, delete_trusted_by}};
use futures::stream::StreamExt;

#[derive(serde::Serialize)]
struct ResponseTrustedBy<T> {
    message: String,
    data: T,
}

pub async fn index(client: web::Data<Client>) -> impl Responder  {
    match get_trusted_by(client).await {
        Ok(mut cursor) => {
            let mut resp = ResponseTrustedBy {
                message: String::from("Success get trusted by"),
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
                        resp.message = String::from("Failed get trusted by");
                    }
                }
            }

            // Build and return the response
            
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Service error: {}", e)),
    }
}

pub async fn update(client: web::Data<Client>, updated_data: web::Json<TrustedBy>) -> impl Responder {
    match &updated_data._id {
        Some(id) => {
            match update_trusted_by(client, updated_data).await {
                Ok(update_res) => {
                    let resp = ResponseTrustedBy {
                        message: String::from("Successfully updated trusted by"),
                        data: Some(update_res),
                    };
                    HttpResponse::Ok().json(resp)
                },
                Err(e) => {
                    let resp: ResponseTrustedBy<Vec<String>> = ResponseTrustedBy {
                        message: e.to_string(),
                        data: vec![],
                    };
                    HttpResponse::Ok().json(resp)
                },
            }
        },
        None => {
            let resp: ResponseTrustedBy<Vec<String>> = ResponseTrustedBy {
                message: String::from("_id is required"),
                data: vec![],
            };
            HttpResponse::Ok().json(resp)
        }
    }
}

pub async fn create(client: web::Data<Client>, created_data: web::Json<TrustedBy>) -> impl Responder {
    let mut resp: ResponseTrustedBy<String> = ResponseTrustedBy {
        message: String::new(),
        data: String::new()
    };

    match &created_data.title {
        Some(title) => if title.is_empty() { resp.message = String::from("Title cannot be empty"); },
        None => { resp.message = String::from("Title cannot be empty"); }
    }

    match &created_data.banner {
        Some(banner) => if banner.is_empty() { resp.message = String::from("Banner cannot be empty"); },
        None => { resp.message = String::from("Banner cannot be empty"); }
    }

    match create_trusted_by(client, created_data).await {
        Ok(created_result) => {
            resp.message = String::from("Trusted by is created successfully");
            resp.data = created_result.inserted_id.to_string();
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}

pub async fn delete(client: web::Data<Client>, deleted_data: web::Json<TrustedBy>) -> impl Responder {
    let mut resp: ResponseTrustedBy<u64> = ResponseTrustedBy {
        message: String::new(),
        data: 0
    };

    match delete_trusted_by(client, deleted_data).await {
        Ok(result) => {
            resp.message = String::from("Trusted by deleted successfully");
            
            if result.deleted_count == 0 {
                resp.message = String::from("Failed to delete trusted by");
            }
            
            resp.data = result.deleted_count;
        },
        Err(err) => {
            resp.message = err.to_string();
        }
    }

    HttpResponse::Ok().json(resp)
}