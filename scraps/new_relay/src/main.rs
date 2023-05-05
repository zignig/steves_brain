#[macro_use]
extern crate rocket;

use chrono::{Datelike, Local, Timelike};
use rocket::serde::{json::Json, Serialize};

mod file_scanner;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Devices 
#[derive(Debug)]
struct Devices { 

}


// Time stuff
#[derive(Debug)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TimeTuple(i32, u32, u32, u32, u32, u32, u32, u32);


impl TimeTuple {
    fn now() -> Self {
        let now = Local::now();
        Self(
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            now.weekday().num_days_from_monday(),
            now.ordinal(),
        )
    }
}

#[get("/time")]
fn get_time() -> Json<TimeTuple> {
    let ar = TimeTuple::now();
    Json(ar)
}

#[get("/status/<name>")]
fn status(name: &str) -> String { 
    file_scanner::get_files();
    format!("Hello, {}",name)    
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_time,status])
}
