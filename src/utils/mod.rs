mod channels_hour_and_status;


use std::fs::*;
use std::io::Read;
use std::collections::HashMap;


pub fn get_channel_name_to_stream() -> HashMap<String, String>{

    let mut file = File::open("info.txt").expect("file not found");

    let mut nametoip = HashMap::new();
    
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    for line in contents.lines() {
        let mut split = line.split(",");
        let name = split.next().unwrap();
        let ip = split.next().unwrap();
        
        nametoip.insert(name.to_string(), ip.to_string());
    }

    return nametoip;
}




pub fn get_channels_and_next_commercial_time() -> Vec<(String, Option<f32>)>{
    write_doc_id_to_file("1vMDw75gHpo2cN0SCyzuB2Yf3JPsEiLmEa5C6AJtDcAA", "./deleteme");
    return channels_hour_and_status::read_channels_and_next_commercial_time("./deleteme/Sheet1.html");
}

use std::path::PathBuf;
use std::io::Cursor;

fn write_doc_id_to_file(documentid: &str, path: &str){

    let url = format!( "https://docs.google.com/spreadsheets/d/{}/export?format=zip&id={}", documentid, documentid);

    let resp = reqwest::blocking::get(url).unwrap();

    let archive = resp.bytes().unwrap();

    let target_dir = PathBuf::from(path); // Doesn't need to exist

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single folder, that folder's contents would be extracted instead.
    zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();
}




pub fn get_current_time() -> f32{
    use chrono::Timelike;
    let now = chrono::Local::now();
    let hour = now.hour();
    let minutes = now.minute();

    let time = hour as f32 + minutes as f32 / 60.0;
    return time;
}


use crate::WINDOWS;

use std::process::Command;
use std::process::Child;


pub fn open_vlc_process( name: &str, _ip: &str ) -> Option<Child>{

    let ip = "/home/jucci/Documents/videoplayback.mp4";

    if WINDOWS{
        if let Ok(childprocess) = Command::new(r#"C:\Program Files (x86)\VideoLAN\VLC\vlc.exe"#)
        .arg(ip)
        .stdout( std::process::Stdio::null() )
        .spawn()
        {
            return Some(childprocess);
        }
    }
    else{

        //uncheck the setting in preferences  "resize interface to video size"
        if let Ok(childprocess) = Command::new(r#"vlc"#)
        .arg(ip)
        .arg( format!(r#"--meta-title="{}""#, name) )
        //.arg("--no-embedded-video")
        // .arg("--no-autoscale")
        // .arg("--no-one-instance")
        // .arg("--height=400")
        // .arg("--width=400")
        // .arg("--video-x=100")
        // .arg("--video-y=100")
        .arg("--qt-minimal-view")
        //.arg("--v4l2-audio-mute")
        .stdout( std::process::Stdio::null() )
        .stderr( std::process::Stdio::null() )
        .spawn()
        {
            return Some(childprocess);
        }


        // .arg("--no-embedded-video")
        // .arg("--no-autoscale")
        // .arg("--no-one-instance")
        // .arg("--height=600")
        // .arg("--width=400")
        // .arg("--video-x=800")
        // .arg("--video-y=800")
        // .arg("--qt-minimal-view")
    }

    return None;
}


