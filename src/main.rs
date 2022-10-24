use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::process::Command;
use std::process::Child;
use std::io;
mod utils;
mod server;


pub struct VLCStreams{

    nametochildprocess: HashMap<String, Child>,
}

impl VLCStreams{

    pub fn new() -> Self{
        let nametochildprocess = HashMap::new();
        let mut x = Self{
            nametochildprocess,
        };
        x.update();
        x
    }

    pub fn update_exited(&mut self){
        

        let mut toremove = HashSet::new();

        for (name, mut handle) in self.nametochildprocess.iter_mut(){

            if let Ok(result) = handle.try_wait(){
                if result.is_some(){
                    toremove.insert(name.clone());
                }
            }
        }

        for name in toremove{
            self.nametochildprocess.remove(&name);
        }
    }


    pub fn update(&mut self){

        let mut unneededprocesses = self.nametochildprocess.keys().map(|a| a.clone() ).collect::<HashSet<_>>();

        for (channelname, url) in utils::get_streams_of_channels_seeking_commercials("1eVeuje_V0GgfDP95Uh9QBJY7xS1BTOAtrY_yM7EgVD0"){

            if ! self.nametochildprocess.contains_key(&channelname){

                let mut handle = Command::new("vlc")
                    .arg("-vvv")
                    .arg(url)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .expect("ls command failed to start");

                self.nametochildprocess.insert(channelname, handle);
            }
            else{
                unneededprocesses.remove(&channelname);
            }
        }


        for channelname in unneededprocesses{

            if let Some(mut handle) = self.nametochildprocess.remove(&channelname){

                handle.kill().unwrap();
            }
        }

    }

}




#[tokio::main]
async fn main() {
    println!("Hello, world!");









    //utils::write_doc_id_to_file( "1eVeuje_V0GgfDP95Uh9QBJY7xS1BTOAtrY_yM7EgVD0", "./fetched" );


    server::serve().await;


    panic!("done");


}




