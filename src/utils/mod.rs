mod channels_hour_and_status;


pub fn get_channel_name_to_type_and_ip() -> HashMap<(String, u32), String>{

    let mut toreturn = HashMap::new();

    for (name, ip) in read_channel_to_udp_stream("receivingstreams.txt"){
        toreturn.insert((name, 0), ip) ;
    }

    for (name, ip) in read_channel_to_udp_stream("sendingstreams.txt"){
        toreturn.insert((name, 1), ip) ;
    }

    return toreturn;
}


pub fn get_channels_and_next_commercial_time() -> Vec<(String, f32)>{
    write_doc_id_to_file("1vMDw75gHpo2cN0SCyzuB2Yf3JPsEiLmEa5C6AJtDcAA", "./deleteme");
    return channels_hour_and_status::read_channels_and_next_commercial_time("./deleteme/Sheet1.html");
}

use std::path::PathBuf;
use std::io::Cursor;

fn write_doc_id_to_file(documentid: &str, path: &str){

    //let documentid = "1eVeuje_V0GgfDP95Uh9QBJY7xS1BTOAtrY_yM7EgVD0";
    let url = format!( "https://docs.google.com/spreadsheets/d/{}/export?format=zip&id={}", documentid, documentid);

    //println!("url {}", url);
    let resp = reqwest::blocking::get(url).unwrap();
    // let mut out = File::create("./main.zip").unwrap();
    // io::copy(&mut resp, &mut out).unwrap();

    //unzip the file and read

    let archive = resp.bytes().unwrap();

    let target_dir = PathBuf::from(path); // Doesn't need to exist

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single folder, that folder's contents would be extracted instead.
    zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();
}



use std::fs::*;
use std::io::Read;
use std::collections::HashMap;

fn read_channel_to_udp_stream( path: &str ) -> HashMap<String, String>{

    let mut file = File::open(path).expect("file not found");

    let mut nametoip = HashMap::new();
    
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    for line in contents.lines() {
        let mut split = line.split(" ");
        let name = split.next().unwrap();
        let ip = split.next().unwrap();
        
        nametoip.insert(name.to_string(), ip.to_string());
    }

    return nametoip;
}


// //get the urls of the channels which want commercials played
// pub fn get_streams_of_channels_seeking_commercials( documentid: &str ) -> Vec<(String, String)>{
//     write_doc_id_to_file(documentid, "./deleteme");
//     let channel_hour_and_status = channels_hour_and_status::read_channels_and_next_commercial_time( "./deleteme/Sheet1.html");//"./fetched/Sheet1.html" );
//     let channel_to_udpstream = channel_to_udp_stream::read_channel_to_udp_stream();
//     let mut toreturn = vec![];
//     for (channelname, hour, status) in channel_hour_and_status{
//         if status == true{
//             println!( "{}, {}, {}", channelname, hour, status);
//             //get the udp stream
//             if let Some(udpstream) = channel_to_udpstream.get(&channelname){
//                 println!("udpstream {}", udpstream);
//                 let url = "udp://@".to_string() + udpstream;
//                 toreturn.push( (channelname, url) );
//             }
//         }
//     }
//     return toreturn;
// }
