use serde::{Serialize, Deserialize};


pub const PATH: &str = "/api/rtvi_reminders";

#[derive(Serialize, Deserialize, Debug)]
pub enum Input{

    GetReminderTimes,
    IsCurrentRTVIAd,
    DismissCurrentRTVIAd,
    SetRTVIPlaylistString(String),
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Output{

    GetReminderTimes(Vec<f32>),
    CurrentRTVIAd,
    None,

}