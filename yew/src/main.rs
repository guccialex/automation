#![recursion_limit="1000"]
use yew::prelude::*;
use serde::{Serialize};
use std::collections::{HashMap};

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

pub enum Msg{
    Error,

    FetchChannels,

    GetChannels( Vec<(String, Option<f32>, bool)>   ),

    ChannelViewChange( String ),

    GetCurrentTime,
    SetCurrentTime(f32),

    OpenPriorityStreams,
    ToggleOpenPriorityStreams,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Properties{
}

use gloo::timers::callback::Interval;

pub struct App{
    channels: Vec<(String, Option<f32>, bool)> ,
    interval: Interval,
    openpriorityinterval: Option<Interval>,
    currenttime: f32,
}

impl Component for App{
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {

        let fetchchannels = ctx.link().callback(|_| Msg::FetchChannels);
        let getcurrenthour = ctx.link().callback(|_| Msg::GetCurrentTime);

        let interval = Interval::new(1_500, move ||  {
            fetchchannels.emit( () );
            getcurrenthour.emit( () );

            gloo::console::log!("tick");
        });

        let _props = ctx.props().clone();
        let toreturn = Self {
            channels: Vec::new(),
            interval: interval,
            currenttime: 0.0,
            openpriorityinterval: None,
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
                            crate::server_get_request::< Vec<(String, Option<f32>, bool)>  >(  "/get_channels" ).await.unwrap()
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
            Msg::OpenPriorityStreams => {

                ctx.link().send_future(
                    async move {
                        crate::server_get_request::< bool >(  "/open_priority_streams" ).await.unwrap();
                        Msg::Error
                    }
                );

            },
            Msg::ToggleOpenPriorityStreams =>{

                if self.openpriorityinterval.is_none(){
                    let openpriorityinterval = ctx.link().callback(|_| Msg::OpenPriorityStreams);
                    let interval = Interval::new(5_000, move ||  {
                        openpriorityinterval.emit( () );
                    });

                    self.openpriorityinterval = Some(interval);
                }
                else{
                    self.openpriorityinterval = None;
                }

            },
            Msg::SetCurrentTime( time ) => {
                self.currenttime = time - 3.7;
            },
            Msg::GetCurrentTime => {
                ctx.link().send_future(
                    async move {
                        Msg::SetCurrentTime( crate::server_get_request::< f32 >(  "/get_current_time" ).await.unwrap() )
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

        
        let mut channeltotogglecallback= HashMap::new();

        for x in self.channels.clone(){
            let name = x.0.clone();
            channeltotogglecallback.insert( name.clone(), ctx.link().callback(move |_| Msg::ChannelViewChange( name.clone() ) ) );
        }


        let openprioritystyle = if self.openpriorityinterval.is_some(){
            "background-color: green;"
        }
        else{
            "background-color: white;"
        };


        let mut orderedchannels = self.channels.clone();
        orderedchannels.sort_by(|(_, a_nextcommercialhour, _), (_, b_nextcommercialhour, _)| {

            if a_nextcommercialhour.is_none() && b_nextcommercialhour.is_none(){
                return std::cmp::Ordering::Equal;
            }
            else if a_nextcommercialhour.is_none(){
                return std::cmp::Ordering::Greater;
            }
            else if b_nextcommercialhour.is_none(){
                return std::cmp::Ordering::Less;
            }

            a_nextcommercialhour.partial_cmp(b_nextcommercialhour).unwrap_or(std::cmp::Ordering::Equal)
        });


        html!{
            <>

                <button onclick={ctx.link().callback(|_x|{ Msg::FetchChannels })}>
                {"fetch channels"}
                </button>

                <button style={openprioritystyle} onclick={ctx.link().callback(|_x|{ Msg::ToggleOpenPriorityStreams })}>
                {"open priority streams"}
                </button>

                {format!("current time: {}", display_time(self.currenttime)   )}

                <table>

                <tr>
                    <th>{"name"}</th>
                    <th>{"next commercial"}</th>
                    <th>{"VLC"}</th>
                </tr>

                {
                    orderedchannels.iter().map(|(name, nextcommercialhour, isopeninvlc)| {

                        let mut style = "".to_string();


                        if let Some(nextcommercialhour) = nextcommercialhour{

                            if self.currenttime >= (nextcommercialhour - 0.25){
                                style += "background-color: #F69A9A;";
                            }
                            else if (self.currenttime + 1.0) >= (nextcommercialhour - 0.25) {
                                style += "background-color: #A4C2F4;";
                            }
                        }

                        let vlcopenstyle = if *isopeninvlc{
                            "background-color: green;"
                        }else{
                            ""
                        };

                        let callback = channeltotogglecallback.get(name).unwrap().clone();

                        html!{
                            <tr style={style}>

                                <td>
                                    {format!("{}", name)}
                                </td>
                                <td>
                                    {
                                        if let Some( nextcommercialhour) = nextcommercialhour{
                                            format!("{}", display_time(*nextcommercialhour) )
                                        }
                                        else{
                                            "-".to_string()
                                        }
                                    }


                                </td>

                                <td>

                                <button onclick={ callback } style={vlcopenstyle}>
                                    {"open in vlc"}
                                </button>

                                </td>

                            </tr>
                        }
                    }).collect::<Html>()
                }
                </table>

            </>
        }
    }
}



fn display_time( time: f32 ) -> String{

    let hour = time.floor() as i32;
    let minute = ((time - hour as f32) * 60.0).round() as i32;

    format!("{}.{}", hour, minute)
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
