

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
