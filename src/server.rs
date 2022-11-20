use actix_web::{get, post, web, App, HttpServer, Responder};


use tokio::sync::Mutex;
use actix_files::Files;
use actix_web::HttpResponse;

use crate::{VLCStreams, Commercials};



// #[get("/get_channels")]
// async fn get_channels(   data: web::Data<Mutex<VLCStreams>>  ) -> impl Responder {

//     data.lock().await.update();

//     //data.lock().await.open_priority_streams( 4 );

//     let mut toreturn: Vec<(String, Option<f32>, bool)> = data.lock().await.get_state();

//     //sort alphabetically
//     toreturn.sort_by(|a, b| a.0.cmp(&b.0));

//     return HttpResponse::Accepted().json(  toreturn  );
// }


#[get("/get_current_time")]
async fn get_current_time(    ) -> impl Responder {


    return HttpResponse::Accepted().json(  crate::get_current_time()  );
}



// #[get("/open_priority_streams")]
// async fn open_priority_streams(   data: web::Data<Mutex<VLCStreams>>  ) -> impl Responder {

//     data.lock().await.update();

//     data.lock().await.open_priority_streams( 6 );

//     return HttpResponse::Accepted().json(  true  );
// }

// #[post("/post_channel_vlc_open")]
// async fn post_channel_vlc_open(  data: web::Data<Mutex<VLCStreams>>, streamname: web::Json<String>  ) -> impl Responder {

//     println!("channel vlc open request: {}", streamname);

//     data.lock().await.toggle_process( &*streamname );

//     return HttpResponse::Accepted().json(  true  );
// }

#[get("/crement/{name}/{amount}")]
async fn crement( data: web::Data<Mutex<Commercials>> , path: web::Path<(String, u32)>) -> impl Responder {

    //data.lock().await.open_priority_streams( 4 );


    if path.1 == 1{
        data.lock().await.increment( &path.0 );
    }
    if path.1 == 0{
        data.lock().await.decrement( &path.0 );
    }

    return HttpResponse::Accepted();
}




#[get("/get_channels")]
async fn get_channels(   data: web::Data<Mutex<Commercials>>  ) -> impl Responder {

    //data.lock().await.open_priority_streams( 4 );

    let mut toreturn: Vec<(String, Option<f32>)> = data.lock().await.channels_to_next_commercial().into_iter().collect();

    //sort alphabetically
    toreturn.sort_by(|a, b| a.0.cmp(&b.0));

    return HttpResponse::Accepted().json(  toreturn  );
}




//#[actix_web::main] // or #[tokio::main]
pub async fn serve() -> std::io::Result<()> {

    let data = web::Data::new(Mutex::new(  crate::Commercials::new() ));

    
    println!("server starting ");

    HttpServer::new(move || {
        App::new()
            .app_data( data.clone() )
    
            .route("/hello", web::get().to(|| async { "Hello World!"  }))
            //.service( get_similar )
            .service(get_channels )
            .service( crement )
            // .service(post_channel_vlc_open )
            // .service(open_priority_streams )
             .service(get_current_time )
            .service(Files::new("/", "./yew/dist/").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 7243))?
    .run()
    .await
}


//create a server shared between users that keeps track of the dates of the things

