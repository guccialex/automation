use actix_web::{get, web, App, HttpServer, Responder};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::sync::Mutex;
use actix_files::Files;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}



//#[actix_web::main] // or #[tokio::main]
pub async fn serve() -> std::io::Result<()> {


    let data = web::Data::new(Mutex::new(  crate::VLCStreams::new() ));


    //every 5 minutes, update

    let dataclone = data.clone();
    tokio::spawn(async move {
        loop{
            tokio::time::sleep(std::time::Duration::from_secs(1*20)).await;
            dataclone.lock().await.update();
        }
    });



    println!("server starting ");

    HttpServer::new(move || {
        App::new()
            .app_data( data.clone() )
    
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            //.service( get_similar )
            .service(greet)
            .service(Files::new("/", "./yew/dist/").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 7243))?
    .run()
    .await
}

