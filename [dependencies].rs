[dependencies]
actix-web = "4.0"
mongodb = "2.0"
tokio = { version = "1", features = ["full"] }


===========================================================================

use actix_web::{web, App, HttpServer, Responder};
use mongodb::{Client, options::ClientOptions};
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Debug, serde::Serialize)]
struct Data {
    name: String,
    age: u8,
}

async fn add_data_to_db() {
    // Connect to MongoDB
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mydatabase");
    let collection = db.collection::<Data>("data");

    // Generate dummy data
    let dummy_data: Vec<Data> = (0..100)
        .map(|i| Data {
            name: format!("Person {}", i),
            age: rand::random::<u8>() % 100,
        })
        .collect();

    // Insert data into MongoDB
    if let Err(err) = collection.insert_many(dummy_data, None).await {
        eprintln!("Error inserting data: {:?}", err);
    } else {
        println!("Added 100 data entries to the database");
    }
}

async fn index() -> impl Responder {
    loop {
        add_data_to_db().await;

        // Delay for 2 seconds
        delay_for(Duration::from_secs(2)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
