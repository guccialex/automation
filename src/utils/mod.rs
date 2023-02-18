



use regex::Regex;

//EVENT NOTE 09:10

pub fn get_name_and_comm_time_slots_from_file_string(fiilestring: &str) -> (String, Vec<f32>){

    let regex = regex::Regex::new(r#"#EVENT NOTE (\d\d):(\d\d)"#).unwrap();

    let mut times: Vec<f32> = Vec::new();

    //get all capture group 1's in the filestring
    let captures = regex.captures_iter(fiilestring);

    for capture in captures{
        let hour = capture.get(1).unwrap().as_str().parse::<f32>().unwrap();
        let minute = capture.get(2).unwrap().as_str().parse::<f32>().unwrap();
        let time = hour + minute / 60.0;
        times.push(time);
    }

    //#LISTNAME BBC_COMM_Nov_18_Anna

    //capture BBC_COMM


    let mut thename = "invalid name".to_string();
    if let Some(name) = Regex::new(r#"#LISTNAME (\w+)_COMM"#).unwrap().captures(fiilestring){
        if let Some(name) = name.get(1){
            thename = name.as_str().to_string();
        }
    }

    return (thename, times);
}

pub fn get_current_time() -> f32{
    use chrono::Timelike;
    let now = chrono::Local::now();
    let hour = now.hour();
    let minutes = now.minute();

    let time = hour as f32 + minutes as f32 / 60.0;
    return time;
}

