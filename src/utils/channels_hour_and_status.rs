use regex::Regex;
use std::fs::File;
use std::io::Read;

//channel name, hour, i
pub fn read_channels_and_hour_and_seekingcommercial(file: &str) -> Vec<(String, f32, bool)>{


    //find </td><td class="s0"></td>
    let mut toreturn = Vec::new();
    
    let mut file = File::open(file).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    
    //for contents in contents.split("tr"){

    //class="s22".+?>(.+?)<.+?<td .+?>(.?.?.?.?.?.?)<

    let re = Regex::new(r#"class="s22".+?>(.+?)<.+?<td .+?>(.?.?.?.?.?.?)<"#   ).unwrap();

    for cap in re.captures_iter(&contents) {

        let name = cap[1].to_string();
        let hour = cap[2].to_string();

        let data = (name, hour.parse::<f32>().unwrap(), false);

        //println!("toreturn {:?}", data);
        //these are the valid ones
        toreturn.push( data );
    }


    let re = Regex::new(r#"class="s19".+?>(.+?)<.+?<td .+?>(.?.?.?.?.?.?)<"#   ).unwrap();

    for cap in re.captures_iter(&contents) {

        let name = cap[1].to_string();
        let hour = cap[2].to_string();

        let data = (name, hour.parse::<f32>().unwrap(), true);
        println!("toreturn {:?}", data);

        toreturn.push( data );
    }



    println!("nl");
    //}


    return toreturn;


    //<td class="s6"></td><td class="s36" dir="ltr">TVA (TV Asia)</td><td class="s23" dir="ltr">17</td>



}
