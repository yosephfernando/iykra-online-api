use mongodb::Client;
use actix_web::web;

/* create mongodb_client connection */
pub async fn mongodb_con() -> web::Data<Client>{
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    print!("uri -> {}", uri);
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    web::Data::new(client)
}

/* can add other db client connection here */