extern crate subprocess;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use subprocess::{Exec, Redirection};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct OnAir {
    onair: bool
}

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let mut server_url = "http://localhost:8000/";

    match args.len() {
        1 => {
            println!("Defaulting server to {}", server_url);
        },
        2 => {
            server_url = &args[1];
        },
        _ => println!("Too many args"),
     }

    let mut last_onair = false;
    loop {
        let onair = match get_onair_status() {
            Ok(v) => v,
            Err(_e) => false,
        };

        if last_onair == onair {
            std::thread::sleep(std::time::Duration::from_millis(30000));
            continue;
        }
        last_onair = onair;
        
        let payload = OnAir {
            onair: onair,
        };

        let _res: OnAir = reqwest::Client::new()
            .post(server_url)
            .json(&payload)
            .send()?
            .json()?;

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

}

fn get_onair_status() -> Result<bool, Box<dyn std::error::Error>> {
    let out = Exec::shell("lsof -i -n | grep -iE '(zoom|meeting)' | grep -i udp")
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();
    Ok(out != "")
}
