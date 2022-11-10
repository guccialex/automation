use std::collections::HashMap;
use std::collections::HashSet;
use std::process::Command;
use std::process::Child;
mod utils;
mod server;


pub fn get_current_hour() -> u32{
    use chrono::Timelike;
    let now = chrono::Local::now();
    let hour = now.hour();
    return hour as u32;
}

fn open_vlc_process( name: &str, ip: &str ) -> Option<Child>{
    if let Ok(childprocess) = Command::new(r#"C:\Program Files (x86)\VideoLAN\VLC\vlc.exe"#)
    .arg(ip)
    .output()
    .spawn(){

        return Some(childprocess);
    }

    return None;
}

pub struct VLCStreams{

    //channel name to the 24 hour time of it's next commercial
    nametocommercial: HashMap<String, f32>,

    //channel name and stream type to it's ip
    nametoip: HashMap<String, String>,

    //ip to process
    iptochildprocess: HashMap<String, Child>,
}

impl VLCStreams{

    pub fn new() -> Self{
        let mut x = Self{
            nametoip: HashMap::new(),
            nametocommercial: HashMap::new(),
            iptochildprocess: HashMap::new(),
        };
        x.update();
        x
    }

    //get the channels, type of stream, and time of next commercial, and if a vlc window is currently open
    pub fn get_state(&self) -> Vec<(String, (Option<f32>, bool), bool)>{

        let currenthour = get_current_hour() as f32;

        let mut toreturn = Vec::new();

        for (name, ip) in self.nametoip.clone(){
            let isopen = self.iptochildprocess.contains_key(&ip);

            if let Some(nextcommercial) = self.nametocommercial.get(&name){

                toreturn.push( (name, (Some(*nextcommercial), &currenthour >= nextcommercial ), isopen) );
            }
            else{
                toreturn.push( (name, (None, false), isopen) );
            }
        }

        return toreturn;
    }

    pub fn toggle_process( &mut self, channelname: &str ){

        let ip = self.nametoip.get(&channelname.to_string()).unwrap().clone();

        if self.iptochildprocess.contains_key(&ip){
            //close the process
            if let Some(childprocess) = self.iptochildprocess.get_mut(&ip){
                childprocess.kill().unwrap();
                self.iptochildprocess.remove(&ip);    
            }

        }else{
            //open the process
            if let Some(childprocess) = open_vlc_process( channelname, &ip ){
                self.iptochildprocess.insert(ip, childprocess);
            }
        }
    }

    pub fn open_priority_streams( &mut self, amount: u32 ){

        let currenthour = get_current_hour() as f32 + 1.0;

        print!("current hour {}", currenthour);
        
        //get the list of priority input streams as ips
        let mut prioritystreams = HashSet::new();

        for (channelname, nextcommercialhour) in self.nametocommercial.clone(){
            if currenthour >= nextcommercialhour {
                if let Some(ip) = self.nametoip.get(&channelname){
                    prioritystreams.insert(ip.clone());
                }
            }
        }

        self.iptochildprocess.retain(|ip, childprocess|{
            if !prioritystreams.contains(ip){
                childprocess.kill().unwrap();
                false
            }
            else{
                true
            }
        });


        //open the streams that are priority until the amount of streams is reached
        for ip in prioritystreams{
            //if its not already opened
            if !self.iptochildprocess.contains_key(&ip){
                if self.iptochildprocess.len() < amount as usize{

                    if let Some(childprocess) = open_vlc_process( &ip, &ip ){
                        self.iptochildprocess.insert(ip, childprocess);
                    }

                }
            }
        }

    }

    pub fn update(&mut self){

        self.nametocommercial = utils::get_channels_and_next_commercial_time().into_iter().collect();
        for ((name, _), ip) in utils::get_channel_name_to_type_and_ip(){
            self.nametoip.insert(name, ip);
        }
    }
}


#[tokio::main]
async fn main() {
    
    //utils::get_channels_and_next_commercial_time();

    //get the IP of the commercials


    
    server::serve().await;
}