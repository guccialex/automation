use regex::Regex;
use std::fs::File;
use std::io::Read;

//channel name, hour of next commercial
pub fn read_channels_and_next_commercial_time(file: &str) -> Vec<(String, Option<f32>)>{

    let mut toreturn = Vec::new();
    
    let mut file = File::open(file).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");



    let re = Regex::new( r#"class=".?.?.?.?" dir=".?.?.?.?.?">(\D.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?.?)</td><td class=".?.?.?.?.?" dir="ltr">(\d.?.?.?.?.?)</td><td cla"# ).unwrap();

    for cap in re.captures_iter(&contents) {

        let name = cap[1].to_string();
        let time = cap[2].to_string();

        if name.contains("hal"){
            println!("name {}", name);
        }


        if let Ok(time) = time.parse::<f32>(){

            let hour = time.floor();
            let minute = (time - hour) * 100.0 ;
            

            let time = hour + minute/60.0;

            let data = (name, Some(time) );

            toreturn.push( data );
        }
        else{
            let data = (name, None);
            toreturn.push( data );
        }
    }

    return toreturn;
}
