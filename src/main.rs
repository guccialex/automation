use std::collections::HashMap;
use std::collections::HashSet;

use std::process::Child;
mod utils;
mod server;

mod commercials;
pub use commercials::Commercials;

pub use utils::get_current_time;
use utils::open_vlc_process;


const WINDOWS: bool = cfg!(target_os = "windows");


pub struct VLCStreams{

    //channel name to the 24 hour time of it's next commercial
    nametocommercial: HashMap<String, f32>,

    //channel name and stream type to it's ip
    nametoip: HashMap<String, String>,

    //ip to process
    iptoprocess: HashMap<String, Child>,
}


impl VLCStreams{

    pub fn new() -> Self{
        let mut x = Self{
            nametoip: HashMap::new(),
            nametocommercial: HashMap::new(),
            iptoprocess: HashMap::new(),
        };
        x.update();
        x
    }

    //get the channel name, the time of the next commercial, and if a vlc window is currently open
    pub fn get_state(&self) -> Vec<(String, Option<f32>, bool)>{

        let mut toreturn = Vec::new();

        for (name, ip) in self.nametoip.clone(){
            let isopen = self.iptoprocess.contains_key(&ip);

            if let Some(nextcommercial) = self.nametocommercial.get(&name){

                toreturn.push( (name, Some(*nextcommercial) , isopen) );
            }
            else{
                toreturn.push( (name, None, isopen) );
            }
        }

        return toreturn;
    }

    pub fn toggle_process( &mut self, channelname: &str ){

        let ip = self.nametoip.get(&channelname.to_string()).unwrap().clone();

        if self.iptoprocess.contains_key(&ip){
            //close the process
            if let Some(childprocess) = self.iptoprocess.get_mut(&ip){
                childprocess.kill().unwrap();
                self.iptoprocess.remove(&ip);    
            }

        }else{
            //open the process
            if let Some(childprocess) = open_vlc_process( channelname, &ip ){
                self.iptoprocess.insert(ip, childprocess);
            }
        }
    }

    pub fn open_priority_streams( &mut self, amount: u32 ){

        //get the list of priority input streams as ips
        let prioritystreams = utils::get_priority(&self.nametocommercial, amount as usize)
            .into_iter()
            .filter_map(|name| {

                if let Some(ip) = self.nametoip.get(&name){
                    return Some( (name, ip.clone()) );
                }
                else{
                    return None;
                }
            })
            .collect::<HashMap<String, String>>();
        
        let priorityips = prioritystreams.iter().map(|x| x.1.clone()).collect::<HashSet<String>>();

        self.iptoprocess.retain(|ip, childprocess|{
            if !priorityips.contains(ip){
                childprocess.kill().unwrap();
                false
            }
            else{
                true
            }
        });


        //open the streams that are priority until the amount of streams is reached
        for (name, ip) in prioritystreams{
            //if its not already opened
            if !self.iptoprocess.contains_key(&ip){

                if let Some(childprocess) = open_vlc_process( &name, &ip ){
                    self.iptoprocess.insert(ip, childprocess);
                }
            }
        }

    }

    pub fn update(&mut self){

        self.nametocommercial = utils::get_channels_and_next_commercial_time().into_iter().filter_map(|(a,optionhour)|{ 
            if let Some(hour) = optionhour{
                Some( (a, hour) )
            }
            else{
                None
            }
         }).collect();

        for (name, ip) in utils::get_channel_name_to_stream(){
            self.nametoip.insert(name, ip);
        }

        let mut exitedprocesses = Vec::new();

        for (ip, process) in &mut self.iptoprocess{

            match process.try_wait() {
                Ok(Some(status)) => {println!("exited with: {status}");  exitedprocesses.push(ip.clone());},
                Ok(None) => { /*still running*/ }
                Err(e) => println!("error attempting to wait: {e}"),
            }
        }

        for x in exitedprocesses{
            self.iptoprocess.remove(&x);
        }
    }
}




//a server that keeps track of the played commercials

//it can also, have, when the commercial was played




use regex::Regex;


#[tokio::main]
async fn main() {


    //read the folder comm_playlists

    //for each get the list of


    

    // let commercials = commercials::Commercials::new();

    // println!( "{:?}", commercials.channels_to_next_commercial());


    //let mut bbc_comm = Vec::new();

    //#EVENT NOTE 08:00
    

    //println!("contents: {}", contents);





    server::serve().await.unwrap();
}