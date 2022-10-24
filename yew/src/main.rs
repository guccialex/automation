#![recursion_limit="1000"]
use yew::prelude::*;
use serde::{Serialize,Deserialize};


#[function_component(Root)]
fn root() -> Html {
    html! {
        < App />
    }
}


fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<Root>();
}


use web_sys::HtmlTextAreaElement;

use std::collections::{HashMap, HashSet};

pub enum Msg{
    
    Error,

    //get what vlcs should be opened

}


#[derive(Clone, PartialEq, Properties)]
pub struct Properties{
}

pub struct App{

}



impl Component for App{
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let _props = ctx.props().clone();
        let toreturn = Self {

        };
        toreturn
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg{
            Msg::Error => {},


        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let _props = ctx.props().clone();
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html{


        use web_sys::HtmlInputElement;



        html!{
            <>


                {"hello"}


            </>
        }
    }
}



//fetch the channels with the 



use serde::de::DeserializeOwned;


pub async fn server_post_request<I: Serialize, O: DeserializeOwned>( message: I, path: &str ) -> Option<O>{

    let url = web_sys::window().unwrap().origin() + path;

    let future = reqwest::Client::new().post(url)
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
