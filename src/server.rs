#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate scroll_phat_hd;

use std::thread;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::env;
use rocket_contrib::json::{Json, JsonValue};
use scroll_phat_hd::display::*;
use scroll_phat_hd::scroller::*;
use rocket::State;

type OnAirState = Arc<Mutex<HashMap<String, bool>>>;

#[derive(Serialize, Deserialize)]
struct OnAir {
    onair: bool
}

enum DisplayType {
    I2C,
    Unicode,
    Term,
    Quiet
}

const ONAIR_STR: &str     = "   ON AIR   ";
const OFFAIR_STR: &str    = "            ";

#[get("/")]
fn index(state_mutex: State<OnAirState>) -> JsonValue {
    let state = state_mutex.lock().unwrap();

    let onair = state.get("onair");
    if let Some(v) = onair {
        json!({
            "status": "success",
            "onair" : v
        })
    } else {
        json!({
            "status": "failed"
        })
    }
}

#[post("/", format = "json", data = "<onair>")]
fn post_on_air(onair: Json<OnAir>, state_mutex: State<OnAirState>) -> JsonValue {
    let mut state = state_mutex.lock().unwrap();
    *state.entry("onair".to_string()).or_insert(onair.onair) = onair.onair;

    json!({
        "status": "success",
        "onair" : onair.onair
    })
}

fn run_display(state_mutex: OnAirState, display_type: DisplayType) {
    let mut display: Box<dyn Display>;
    match display_type {
        DisplayType::Quiet => { return; }
        _ => {
            display = match display_type {
                #[cfg(target_os = "linux")]
                DisplayType::I2C => Box::new(I2CDisplay::new(1)),
                DisplayType::Term => Box::new(TermDisplay::new()),
                DisplayType::Unicode => Box::new(UnicodeDisplay::new()),
                _ => Box::new(UnicodeDisplay::new()),
            };
        }
    }
    let mut scroller = Scroller::new(&mut (*display));

    scroller.set_text(OFFAIR_STR);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        scroller.scroll();
        let state = state_mutex.lock().unwrap();
        scroller.show();
        let onair = state.get("onair");
        
        if let Some(v) = onair {
            if *v == true {
                scroller.set_text(ONAIR_STR);
            } else {
                scroller.set_text(OFFAIR_STR);
            }
        } else {
            scroller.set_text(OFFAIR_STR);
        }
        std::mem::drop(state);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let state_mutex: OnAirState = Arc::new(Mutex::new(HashMap::<String, bool>::new()));
    let state_mutex1 = state_mutex.clone();
    let state_mutex2 = state_mutex.clone();
    let mut display_type = DisplayType::Unicode;

     match args.len() {
        1 => {
            println!("Defaulting to local Unicode display. ");
        },
        2 => {
            let display_type_arg = &args[1];
            match &display_type_arg[..] {
                #[cfg(target_os = "linux")]
                "--I2C" => {
                    println!("Using I2C display");
                    display_type = DisplayType::I2C;
                },
                "--term" => {
                    println!("Using terminal display");
                    display_type = DisplayType::Term;
                },
                "--unicode" => {
                    println!("Using Unicode display");
                    display_type = DisplayType::Unicode;
                },
                "--quiet" => {
                    println!("Using no display");
                    display_type = DisplayType::Quiet;
                },
                _ => println!("Unkown arg {} ", args[1]),
            }
        },
        _ => println!("Too many args"),
     }

    std::thread::sleep(std::time::Duration::from_millis(1000));
    thread::spawn(move || run_display(state_mutex1, display_type));
    rocket::ignite()
        .mount("/", routes![index, post_on_air])
        .manage(state_mutex2)
        .launch();
}