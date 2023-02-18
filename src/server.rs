use actix_web::{web, App, HttpServer, Responder};


use tokio::sync::Mutex;
use actix_files::Files;
use actix_web::HttpResponse;

use crate::{ChannelsToCommercials, RTVIReminders};



async fn get_current_time(  ) -> impl Responder {

    let output: shared::backends::getcurrenttime::Output = crate::get_current_time();
    return HttpResponse::Accepted().json( output );
}


async fn crement( data: web::Data<Mutex<ChannelsToCommercials>> , input: web::Json< shared::crementcommercials::Input  >) -> impl Responder {

    let input = input.0;

    data.lock().await.crement( &input.0, input.1 );

    return HttpResponse::Accepted();
}


async fn get_commercials(   data: web::Data<Mutex<ChannelsToCommercials>>  ) -> impl Responder {

    let mut toreturn: shared::getcommercials::Output = data.lock().await.get_channels_to_current_commercial_hour();

    //sort alphabetically
    toreturn.sort_by(|a, b| a.0.cmp(&b.0));

    return HttpResponse::Accepted().json(  toreturn  );
}



async fn rtvi_reminders(   data: web::Data<Mutex<RTVIReminders>> , input: web::Json< shared::rtvireminders::Input  > ) -> impl Responder {

    
    use shared::rtvireminders::Input;
    use shared::rtvireminders::Output;

    let output: Output = match input.0{

        Input::DismissCurrentRTVIAd => {
            data.lock().await.dismiss_current_reminder();
            Output::None
        },
        Input::GetReminderTimes => {
            let mut toreturn = data.lock().await.get_reminder_times();
            toreturn.sort_by(|a, b| a.partial_cmp(&b).unwrap() );
            Output::GetReminderTimes( toreturn )
        },
        Input::IsCurrentRTVIAd => {
            let toreturn = data.lock().await.is_current_reminder();
            
            println!("is current reminder: {}", toreturn);
            if toreturn{
                Output::CurrentRTVIAd
            }
            else{   
                Output::None
            }
        },
        Input::SetRTVIPlaylistString( rtvistring ) => {
            data.lock().await.load_rtvi_ads( &rtvistring );
            Output::None
        },
    };


    return HttpResponse::Accepted().json(  output  );

}

//     let mut toreturn: shared::iscurrentrtviad::Output = data.lock().await.is_current_reminder();

//     return HttpResponse::Accepted().json(  toreturn  );
// }

// async fn set_rtvi_playlist_string(   data: web::Data<Mutex<RTVIReminders>> , input: web::Json< shared::setrtviplayliststring::Input  >  ) -> impl Responder {

//     let input = input.0;

//     data.lock().await.load_rtvi_ads( &input );

//     return HttpResponse::Accepted();
// }


// async fn dismiss_current_rtvi_ad(   data: web::Data<Mutex<RTVIReminders>>  ) -> impl Responder {

//     data.lock().await.dismiss_current_reminder();

//     return HttpResponse::Accepted();
// }






//#[actix_web::main] // or #[tokio::main]
pub async fn serve() -> std::io::Result<()> {

    let data = web::Data::new(Mutex::new(  crate::Commercials::new() ));

    let reminders = web::Data::new(Mutex::new(  crate::RTVIReminders::new() ));

    
    println!("server starting ");

    HttpServer::new(move || {
        App::new()
            .app_data( data.clone() )
            .app_data( reminders.clone() )
    
            .route("/hello", web::get().to(|| async { "Hello World!"  }))

            .service( web::resource( shared::getcurrenttime::PATH ).route( web::get().to( get_current_time)   ) )
            .service( web::resource( shared::getcommercials::PATH ).route( web::get().to( get_commercials )   ) )
            .service( web::resource( shared::crementcommercials::PATH ).route(web::post().to( crement )) )


            .service( web::resource( shared::rtvireminders::PATH ).route( web::post().to( rtvi_reminders )   ) )

            .service(Files::new("/", "./yew/dist/").index_file("index.html"))
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


//create a server shared between users that keeps track of the dates of the things


// .service( web::resource( shared::backends::getexplanitoryfactors::PATH ).route(web::post().to(  get_explanitory_factors   )) )
// .service( web::resource( shared::backends::getgenres::PATH).route( web::get().to(get_genres) ) )