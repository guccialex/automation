

use std::fs;
use std::collections::HashMap;
use regex::Regex;


//the struct that contains when the commercials should be played and when they are
#[derive(Debug, Clone)]
pub struct Commercials{

    channelstocommercials: HashMap<String, Vec<f32>>,

    channelstonextcomm: HashMap<String, usize>,

}

impl Commercials{

    pub fn new() -> Self{

        let mut channelstocommercials = HashMap::new();
        let mut channelstonextcomm = HashMap::new();

        let paths = fs::read_dir("./comm_playlists").unwrap();

        for path in paths {

            let path = path.unwrap().path().display().to_string();

            let contents = fs::read_to_string(path.clone()).expect("err");

            let re = Regex::new( r#"EVENT NOTE (..):(..)\r"# ).unwrap();

            let mut times = Vec::new();

            for cap in re.captures_iter(&contents) {

                let hour = cap[1].parse::<f32>().unwrap();
                let minute = cap[2].parse::<f32>().unwrap();

                times.push( hour + minute/60.0 );

            }

            let channelname = path.split("/").last().unwrap().split(".").next().unwrap().to_string();

            let re = Regex::new(r"(.?.?.?.?.?)_").expect("failed to compile regex");

            
            let channelname = re.captures_iter(&channelname).map(|x| x[1].to_string()  ).next().unwrap_or("badname".to_string());

            channelstocommercials.insert(channelname.clone(), times);
            channelstonextcomm.insert(  channelname, 0  );
        }

        Self{
            channelstocommercials,
            channelstonextcomm,
        }
    }


    pub fn increment(&mut self, channelname: &str){
        if let Some(currentcomm) = self.channelstonextcomm.get_mut(channelname){
            *currentcomm += 1;
        }
    }

    pub fn decrement(&mut self, channelname: &str){
        if let Some(currentcomm) = self.channelstonextcomm.get_mut(channelname){
            *currentcomm -= 1;
        }
    }


    pub fn channels_to_next_commercial(&self) -> HashMap<String, Option<f32>>{

        let mut toreturn = HashMap::new();

        for (channelname, commercials) in &self.channelstocommercials{

            if let Some(currentcomm) = self.channelstonextcomm.get(channelname){

                if *currentcomm < commercials.len(){
                    toreturn.insert(channelname.clone(), Some(commercials[*currentcomm]));
                }
                else{
                    toreturn.insert(channelname.clone(), None);
                }
            }
        }

        toreturn
    }



}


