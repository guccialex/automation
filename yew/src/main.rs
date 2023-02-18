#![recursion_limit="1000"]
use yew::prelude::*;
use serde::{Serialize};


mod uploadfile;
mod app;


#[function_component(Root)]
fn root() -> Html {
    html! {
        < app::App />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Root>::new().render();
}









use serde::de::DeserializeOwned;
use core::fmt::Debug;

pub async fn server_post_request<I: Serialize + Debug, O: DeserializeOwned>( message: I, path: &str ) -> Option<O>{

    let url = web_sys::window().unwrap().origin() + path;

    gloo::console::log!( format!("{:?}   {}", message, url) );

    let future = reqwest::Client::new()
        .post(url)
        .json( &message )
        .send();

    if let Ok(response) = future.await{

        match  response.json::< O >().await{
            Ok( x ) =>{
                return Some( x );
            },
            Err (x) => {
                let x = format!("the error message I got: {:?}", x );
                gloo::console::log!("response", x);
            }
        }
    }
    
    return None;
} 



pub async fn server_get_request< O: DeserializeOwned>(path: &str ) -> Option<O>{
    let url = web_sys::window().unwrap().origin() + path;

    let future = reqwest::Client::new().get(url)
        .send();

    if let Ok(response) = future.await{
        match  response.json::< O >().await{
            Ok( x ) =>{
                return Some( x );
            },
            Err (x) => {
                let x = format!("the error message I got: {:?}", x );
                gloo::console::log!("response", x);
            }
        }
    }
    
    return None;
} 
