extern crate ms_speaker_recog;

use ms_speaker_recog::Client;

fn main() {
    let client = Client::new(include_str!("../api.txt"), 1);
    let res = client.create_profile("en-us");
    match res {
        Ok(s) => println!("Success {}", s),
        Err(err) => println!("{}", err.message)
    }
}
