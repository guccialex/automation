

//0 for red, 1 for blue, 2 for white
pub fn get_priority_level( currenttime: f32, nextcommercialtime: f32) -> u8{

    if currenttime >= (nextcommercialtime - 0.25){
        return 0;
    }
    else if (currenttime + 1.0) >= (nextcommercialtime - 0.25) {
        return 1;
    }

    return 2;
}


pub mod backends;
pub use backends::*;



pub fn display_time( time: f32 ) -> String{

    let hour = time.floor() as i32;
    
    let minute = ((time - hour as f32) * 60.0).round() as i32;



    format!("{}.{}", hour, minute)
}




