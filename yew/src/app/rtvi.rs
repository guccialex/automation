use yew::prelude::*;
use std::collections::{HashMap};
use shared::rtvireminders;
use tts::Tts;
use gloo::file::File;

pub enum Msg{
    Error,

    IsCurrentReminder,

    DismissReminder,

    GetReminderTimes,

    SetReminderTimes( Vec<f32> ),

    Update,

    Speak,

    UploadRTVIstring( String ),

    File( File ),

}


#[derive(Clone, PartialEq, Properties)]
pub struct Properties{
    pub currenttime: f32,
}

use gloo::timers::callback::Interval;
use wasm_bindgen::JsCast;


pub struct RTVI{
    reminders: Vec<f32>,
    updateinterval: Interval,
    tts: Tts,

}

impl Component for RTVI{
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {

        let mut tts = Tts::default().unwrap();
        if tts.voices().unwrap().iter().len() > 0 {
            tts.set_voice(&tts.voices().unwrap()[1]).expect("Failed to set voice");
        }

        tts.set_pitch( 0.8 );
    

        let update = ctx.link().callback(|_| Msg::Update);
        update.emit( () );

        let updateinterval = Interval::new(3000, move ||  {
            update.emit( () );
        });



    
        let _props = ctx.props().clone();
        let toreturn = Self {
            updateinterval,
            tts,
            reminders: Vec::new(),
        };

        toreturn
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg{
            Msg::Error => {},

            Msg::Update =>{
                ctx.link().send_future(
                    async move {
                        Msg::IsCurrentReminder
                    }
                );

                ctx.link().send_future(
                    async move {
                        Msg::GetReminderTimes
                    }
                );
            },
            Msg::IsCurrentReminder =>{
              
                ctx.link().send_future(
                    async move {
                        if let rtvireminders::Output::CurrentRTVIAd = crate::server_post_request::< rtvireminders::Input, rtvireminders::Output  >( rtvireminders::Input::IsCurrentRTVIAd, shared::rtvireminders::PATH ).await.unwrap(){
                            Msg::Speak
                        }
                        else{
                            Msg::Error
                        }
                    }
                );
                
            },
            Msg::DismissReminder =>{
                ctx.link().send_future(
                    async move {
                        crate::server_post_request::< rtvireminders::Input, rtvireminders::Output  >( rtvireminders::Input::DismissCurrentRTVIAd, shared::rtvireminders::PATH ).await.unwrap();
                        Msg::Error
                    }
                );
            },
            Msg::GetReminderTimes =>{

                ctx.link().send_future(
                    async move {
                        if let rtvireminders::Output::GetReminderTimes(times) = crate::server_post_request::< rtvireminders::Input, rtvireminders::Output  >( rtvireminders::Input::GetReminderTimes, shared::rtvireminders::PATH ).await.unwrap(){
                            Msg::SetReminderTimes( times )
                        }
                        else{
                            Msg::Error
                        }
                    }
                );
            },
            Msg::SetReminderTimes( times ) =>{
                self.reminders = times;
            },
            Msg::Speak => {
               gloo::console::log!("rtvi ad");
                self.tts.speak("rtvi", true).unwrap();
            },
            Msg::File(file) => {
                ctx.link().send_future(
                    async move {
                        let string = gloo_file::futures::read_as_text(&file).await;
                        Msg::UploadRTVIstring( string.unwrap_or_else(|e| e.to_string() ) )
                    }
                );
            },
            Msg::UploadRTVIstring( string ) => {
                ctx.link().send_future(
                    async move {
                        crate::server_post_request::< rtvireminders::Input, rtvireminders::Output  >( rtvireminders::Input::SetRTVIPlaylistString( string ), shared::rtvireminders::PATH ).await.unwrap();
                        Msg::Error
                    }
                );
            },
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool{
        let _props = ctx.props().clone();

        //self.update(ctx, rtvireminders::Input::IsCurrentRTVIAd );

        false
    }

    fn view(&self, ctx: &Context<Self>, ) -> Html{


        html!{
            <>



                {
                    self.reminders.iter().map(|(time)| {

                        let mut display = shared::display_time( *time );
                        //let display = format!("{:?}", time);

                        html!{
                            <div>
                                {display}
                                <br/>
                            </div>
                        }
                    }).collect::<Html>()
                }

                <button onclick={ctx.link().callback(|_x|{ Msg::DismissReminder })}>
                {"dismiss reminder"}
                </button>

                
                <br/>
                <br/>

                <input type="file" multiple=true onchange={
                    
                ctx.link().callback(move |e: Event| {
                
                    let mut result = Vec::new();
                    let input: HtmlInputElement = e.target_unchecked_into();

                    if let Some(files) = input.files() {
                        let files = js_sys::try_iter(&files)
                            .unwrap()
                            .unwrap()
                            .map(|v| web_sys::File::from(v.unwrap()))
                            .map(File::from);
                        result.extend(files);
                    }

                    if result.len() > 0{
                        Msg::File(result[0].clone())
                    }
                    else{
                        Msg::Error
                    }

                })
                }

                />
            
            </>
        }
    }
}

use web_sys::HtmlInputElement;

fn display_time( time: f32 ) -> String{

    let hour = time.floor() as i32;
    let minute = ((time - hour as f32) * 60.0).round() as i32;

    format!("{}.{}", hour, minute)
}

















