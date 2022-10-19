use std::collections::HashMap;
use std::env;


//read a text file that has name to ip

use std::process::Command;

fn main() {
    println!("Hello, world!");


    //let mut nametoip = HashMap::new();



    //ip addresses to the names of the program as on the google sheet

    //google sheet 

    println!("{:?}", read_name_to_ip() );

    let nametoip = read_name_to_ip();

    let urlprefix = "http://".to_string();


    read_channels_and_hour_and_status(  "Sheet1.html" );

    panic!("done");


    //loop

    //every 20 seconds

    //check the url

    Command::new("vlc")
            .arg("-vvv")
            .arg("rtsp://wowzaec2demo.streamlock.net/vod/mp4:BigBuckBunny_115k.mp4")
            .spawn()
            .expect("ls command failed to start");


}

use std::fs::*;
use std::io::Read;


fn read_name_to_ip() -> HashMap<String, String>{

    //read the file called ips.txt

    let mut nametoip = HashMap::new();

    let mut file = File::open("ips.txt").expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    //iterate through each line
    
    for line in contents.lines() {
        let mut split = line.split(" ");
        let name = split.next().unwrap();
        let ip = split.next().unwrap();
        
        nametoip.insert(name.to_string(), ip.to_string());

    }
    

    return nametoip;

    

}


use regex::Regex;

fn read_channels_and_hour_and_status(file: &str) -> Vec<(String, u32, bool)>{


    //find </td><td class="s0"></td>
    let mut toreturn = Vec::new();
    
    let mut file = File::open(file).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    //for contents in contents.split("tr"){

        let re = Regex::new(r#"<td class="s." dir="(.+?)">(.+?)</td>"#   ).unwrap();

        for cap in re.captures_iter(&contents) {
            let channel = cap[2].to_string();

            let data = (channel, 0, true);
            println!("toreturn {:?}", data);

            toreturn.push( data );
        }
    
    

        println!("nl");
    //}


    return toreturn;



}





//at the start of each hour
//open the VLC streams of the channels that are supposed to have ads played this hour
//and then remove them when the items change to black
//open the vlc stream that 
