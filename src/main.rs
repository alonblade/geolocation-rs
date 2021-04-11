#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

use std::process::Command;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::SystemTime;
use std::env;

// Example request from firefox:
// POST / HTTP/1.1
// Host: 127.0.0.1:9999
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0
// Accept: */*
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate
// Content-Type: application/json; charset=UTF-8
// Origin: null
// Content-Length: 78
// Connection: keep-alive
// 
// {"wifiAccessPoints":[{"macAddress":"78-0c-f0-58-78-6d","signalStrength":-66}]}

#[post("/")]

fn index() -> content::Json<&'static str> {
    println!("requested position");
    Command::new("notify-send")
        .arg("position-requested")
        .output().unwrap();

    let location = format!("{}/.geolocation_rs_history", env::var("HOME").unwrap());

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(location)
        .unwrap();
    let now = SystemTime::now();

    if let Err(_e) = writeln!(file, "{:?}", now) {
        // ignore errors
    }

    // I really want unwrap_or_else and just silently ignore but I don't know how...
    content::Json(r#"{
  "location": {
    "lat": 32.81780,
    "lng": 35.00208
  },
  "accuracy": 10
}"#)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
