use chrono::Local;
use crate::models::Clip;  //import struct from models.rs

pub fn handle(clip: &Clip, input: &str) -> bool {
    match input.trim().to_lowercase().as_str() {
        "hello" => {
            println!(" Hi! I'm {} your assistant. Please use Help to see a list of Commands.", clip.name)
        }
        "showhour" => {
            let now = Local::now();
            pirntln!("{}: Today is {}", clip.name, now.format("%Y-%m-%d %H:%M:%S"));
        }
        "exit" => return false,
        _=> println!("Unknown command. type 'help'."),
    }
    true
}