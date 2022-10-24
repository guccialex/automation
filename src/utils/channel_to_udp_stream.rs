
use std::fs::*;
use std::io::Read;
use std::collections::HashMap;

pub fn read_channel_to_udp_stream() -> HashMap<String, String>{

    let mut nametoip = HashMap::new();

    let mut file = File::open("ips.txt").expect("file not found");

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
