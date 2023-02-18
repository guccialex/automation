use std::collections::HashMap;



mod utils;
mod server;

mod commercials;
pub use commercials::Commercials;

pub use utils::get_current_time;


pub struct ChannelsToCommercials{
    pub channelstocurrent: HashMap<String, usize>,
    pub channeltocommercials: HashMap<String, Vec<f32>>,
}

impl ChannelsToCommercials{
    pub fn new() -> Self{
        ChannelsToCommercials{
            channelstocurrent: HashMap::new(),
            channeltocommercials: HashMap::new(),
        }
    }

    pub fn load_commercials(&mut self, filestring: &str){
        let (name, comms) = utils::get_name_and_comm_time_slots_from_file_string(filestring);
        self.channeltocommercials.insert(name, comms);
    }

    pub fn crement( &mut self, channel: &str, amount: i32){
        let current = self.channelstocurrent.get_mut(channel).unwrap();
        *current = (*current as i32 + amount) as usize;
    }

    pub fn get_channels_to_current_commercial_hour(&self) -> Vec<(String, f32)>{

        let mut toreturn: Vec<(String, f32)> = Vec::new();

        for (channel, currentslot) in self.channelstocurrent.iter(){
            if let Some(nexthour) = self.channeltocommercials.get(channel).unwrap().get(*currentslot){
                toreturn.push( (channel.clone(), *nexthour) );
            }
        }

        return toreturn;
    }


}





pub struct RTVIReminders{
    adtimes: Vec<f32>,
    hoursbefore: f32,
}

impl RTVIReminders{
    pub fn new() -> Self{
        RTVIReminders{
            adtimes: Vec::new(),
            hoursbefore: 2. / 60.,
        }
    }

    pub fn get_reminder_times(&self) -> Vec<f32>{
        let mut toreturn = self.adtimes.clone();
        for time in toreturn.iter_mut(){
            *time -= self.hoursbefore;
        }
        return toreturn;
    }

    pub fn load_rtvi_ads( &mut self, rtvistring: &str) {
        self.adtimes = utils::get_name_and_comm_time_slots_from_file_string(rtvistring).1;
    }

    pub fn dismiss_current_reminder(&mut self){
        if self.is_current_reminder() {
            self.adtimes.remove(0);
        }
    }

    pub fn is_current_reminder(&self) -> bool{

        if self.adtimes.len() == 0{
            return false;
        }


        let currenttime = get_current_time();

        if self.adtimes[0] <= currenttime + self.hoursbefore{
            return true;
        }
        return false;
    }

}



#[tokio::main]
async fn main() {

    server::serve().await.unwrap();
}