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

    SetChannels( Vec<(String, Option<f32>)>   ),

    GetCurrentTime,
    SetCurrentTime(f32),

    Crement( String, u32 ),

}


#[derive(Clone, PartialEq, Properties)]
pub struct Properties{
}

use gloo::timers::callback::Interval;

pub struct App{
    channels: Vec<(String, Option<f32>)> ,
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
        fetchchannels.emit( () );
        getcurrenthour.emit( () );

        let interval = Interval::new(500, move ||  {
            fetchchannels.emit( () );
            getcurrenthour.emit( () );
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
            Msg::SetChannels( channels ) => {
                self.channels = channels;
            },
            Msg::FetchChannels =>{
                ctx.link().send_future(
                    async move {
                        Msg::SetChannels( 
                            crate::server_get_request::< Vec<(String, Option<f32>)>  >(  "/get_channels" ).await.unwrap()
                        )
                    }
                );
            },
            Msg::SetCurrentTime( time ) => {
                self.currenttime = time;
            },
            Msg::GetCurrentTime => {
                ctx.link().send_future(
                    async move {
                        Msg::SetCurrentTime( crate::server_get_request::< f32 >(  "/get_current_time" ).await.unwrap() )
                    }
                );
            },
            Msg::Crement( name, amount ) =>{


                let path = format!("/crement/{}/{}", name, amount);

                ctx.link().send_future(
                    async move {
                        
                        crate::server_get_request::< () >(  &path ).await.unwrap();

                        Msg::FetchChannels
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

        for (name, _) in &self.channels{
            let namecopy = name.clone();
            
            let callback = ctx.link().callback(move |_| Msg::Crement( namecopy.clone(), 1 ) );

            channeltotogglecallback.insert( name.to_string(), callback );
        }



        html!{
            <>
                <button onclick={ctx.link().callback(|_x|{ Msg::FetchChannels })}>
                {"fetch channels"}
                </button>

                {format!("current time: {}", display_time(self.currenttime)   )}

                <table>

                <tr>
                    <th>{"name"}</th>
                    <th>{"next commercial"}</th>
                    <th>{"increment"}</th>
                </tr>

                {
                    self.channels.iter().map(|(name, nextcommercialhour)| {

                        let mut style = "".to_string();


                        if let Some(nextcommercialhour) = nextcommercialhour{

                            match shared::get_priority_level(self.currenttime, *nextcommercialhour ){
                                0 => style += "background-color: #F69A9A;",
                                1 => style += "background-color: #A4C2F4;",
                                _ => {},
                            }                        

                        }



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

                                <button onclick={ channeltotogglecallback.get(name).unwrap() } >
                                    {"increment"}
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
