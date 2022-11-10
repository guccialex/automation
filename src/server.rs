use actix_web::{get, web, App, HttpServer, Responder};


use tokio::sync::Mutex;
use actix_files::Files;
use actix_web::HttpResponse;

use crate::VLCStreams;



#[get("/get_channels")]
async fn get_channels(   data: web::Data<Mutex<VLCStreams>>  ) -> impl Responder {

    data.lock().await.update();

    data.lock().await.open_priority_streams( 4 );

    let mut toreturn: Vec<(String, (Option<f32>, bool), bool)> = data.lock().await.get_state();

    //sort alphabetically
    toreturn.sort_by(|a, b| a.0.cmp(&b.0));

    return HttpResponse::Accepted().json(  toreturn  );
}

#[get("/post_channel_vlc_open")]
async fn post_channel_vlc_open(  data: web::Data<Mutex<VLCStreams>>, streamname: web::Json<String>  ) -> impl Responder {

    println!("channel vlc open request: {}", streamname);

    data.lock().await.toggle_process( &*streamname );

    return HttpResponse::Accepted().json(  true  );
}


//#[actix_web::main] // or #[tokio::main]
pub async fn serve() -> std::io::Result<()> {

    let data = web::Data::new(Mutex::new(  crate::VLCStreams::new() ));

    
    println!("server starting ");

    HttpServer::new(move || {
        App::new()
            .app_data( data.clone() )
    
            .route("/hello", web::get().to(|| async { "Hello World!"  }))
            //.service( get_similar )
            .service(get_channels )
            .service(post_channel_vlc_open )
            .service(Files::new("/", "./yew/dist/").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 7243))?
    .run()
    .await
}

