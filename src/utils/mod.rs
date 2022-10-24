mod channels_hour_and_status;

mod channel_to_udp_stream;





//get the urls of the channels which want commercials played
pub fn get_streams_of_channels_seeking_commercials( documentid: &str ) -> Vec<(String, String)>{

    write_doc_id_to_file(documentid, "./deleteme");

    let channel_hour_and_status = channels_hour_and_status::read_channels_and_hour_and_seekingcommercial( "real.html");//"./fetched/Sheet1.html" );

    let channel_to_udpstream = channel_to_udp_stream::read_channel_to_udp_stream();


    let mut toreturn = vec![];

    for (channelname, hour, status) in channel_hour_and_status{

        if status == true{

            println!( "{}, {}, {}", channelname, hour, status);

            //get the udp stream
            if let Some(udpstream) = channel_to_udpstream.get(&channelname){

                println!("udpstream {}", udpstream);

                let url = "udp://@".to_string() + udpstream;


                toreturn.push( (channelname, url) );
            }
        }
    }

    return toreturn;

}


use std::path::PathBuf;
use std::io::Cursor;

fn write_doc_id_to_file(documentid: &str, path: &str){


    //let documentid = "1eVeuje_V0GgfDP95Uh9QBJY7xS1BTOAtrY_yM7EgVD0";

    let url = format!( "https://docs.google.com/spreadsheets/d/{}/export?format=zip&id={}", documentid, documentid);

    println!("url {}", url);

    let mut resp = reqwest::blocking::get(url).unwrap();
    // let mut out = File::create("./main.zip").unwrap();
    // io::copy(&mut resp, &mut out).unwrap();


    //unzip the file and read

    let archive = resp.bytes().unwrap();

    let target_dir = PathBuf::from(path); // Doesn't need to exist

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single folder, that folder's contents would be extracted instead.
    zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();


}