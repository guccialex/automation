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

    FetchChannels,

    //the name of the channel
    //if it's sending (0) if its receiving (1)
    GetChannels( Vec<(String, (Option<f32>, bool), bool)>  ),

    //toggle channel
    //the name, if its the input (0) or output (1), enable or disable
    ChannelViewChange( String ),

}


#[derive(Clone, PartialEq, Properties)]
pub struct Properties{
}


use gloo::timers::callback::Interval;

pub struct App{

    channels: Vec<(String, (Option<f32>, bool), bool)>,
    interval: Interval,

}



impl Component for App{
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {

        let callback = ctx.link().callback(|_| Msg::FetchChannels);

        let interval = Interval::new(1_000, move ||  {
            callback.emit( () );
            gloo::console::log!("tick");
        });

        let _props = ctx.props().clone();
        let toreturn = Self {
            channels: Vec::new(),
            interval: interval,
        };

        toreturn
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg{
            Msg::Error => {},
            //get the list of channels that are actively displayed
            Msg::GetChannels( channels ) => {
                self.channels = channels;
            },
            Msg::FetchChannels =>{
                ctx.link().send_future(
                    async move {
                        Msg::GetChannels( 
                            crate::server_get_request::< Vec<(String,  (Option<f32>, bool), bool)> >(  "/get_channels" ).await.unwrap()
                        )
                    }
                );
            },
            Msg::ChannelViewChange( input ) => {

                ctx.link().send_future(
                    async move {
                        crate::server_post_request::< String, bool >( input, "/post_channel_vlc_open" ).await.unwrap();
                        Msg::Error
                    }
                );
            },

        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let _props = ctx.props().clone();
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html{

        use web_sys::HtmlInputElement;

        let mut channeltotogglecallback= HashMap::new();

        for x in self.channels.clone(){
            let name = x.0.clone();
            channeltotogglecallback.insert( name.clone(), ctx.link().callback(move |_| Msg::ChannelViewChange( name.clone() ) ) );
        }
        

        html!{
            <>

                //an array of buttons

                <button onclick={ctx.link().callback(|x|{ Msg::FetchChannels })}>
                {"fetch channels"}
                </button>
                 


                {
                    self.channels.iter().map(|(name, (nextcommercialhour, playnow), isopeninvlc)| {



                        let mut style = "".to_string();


                        if *playnow{
                            style += "background-color: green;";
                        }

                        let vlcopenstyle = if *isopeninvlc{
                            "background-color: green;"
                        }else{
                            ""
                        };

                        let callback = channeltotogglecallback.get(name).unwrap().clone();

                        html!{
                            <div style={style}>
                                {format!( "{}   {:?}  {}", name, nextcommercialhour, isopeninvlc )}

                                <button onclick={ callback } style={vlcopenstyle}>
                                {"open in vlc"}
                                </button>

                                <br/>
                            </div>
                        }
                    }).collect::<Html>()
                }


            </>
        }
    }
}





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
