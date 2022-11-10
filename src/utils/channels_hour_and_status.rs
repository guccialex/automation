use regex::Regex;
use std::fs::File;
use std::io::Read;

//channel name, hour of next commercial
pub fn read_channels_and_next_commercial_time(file: &str) -> Vec<(String, f32)>{

    let mut toreturn = Vec::new();
    
    let mut file = File::open(file).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");


    //let re = Regex::new( r#">(.?.?.?.?.?.?)</td><td class=".?.?.?" dir="ltr">(.?.?.?.?.?)<.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?"# ).unwrap();

    let re = Regex::new( r#"class=".?.?.?.?" dir=".?.?.?.?.?">(\D.?.?.?.?)</td><td class=".?.?.?.?.?" dir="ltr">(\d.?.?.?.?.?)</td><td cla"# ).unwrap();

    // class=".?.?.?.?" dir=".?.?.?.?.?">(.?.?.?.?.?)</td><td class=".?.?.?.?.?" dir="ltr">(\d.?.?.?.?.?)</td><td cla

    for cap in re.captures_iter(&contents) {

        let name = cap[1].to_string();
        let hour = cap[2].to_string();

        // println!("{}", cap[0].to_string());
        // println!("{}, {}", name, hour);

        if let Ok(hour) =  hour.parse::<f32>(){

            let data = (name, hour );
            //println!("toreturn {:?}", data);
            toreturn.push( data );
        }
    }

    return toreturn;
}
