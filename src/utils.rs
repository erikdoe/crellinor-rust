use std::fs;
use std::fs::File;
use std::io::Write;
//use chrono::prelude::*;
use uuid::Uuid;

const OUTPUT_DIR: &str = "output";

pub fn write_logfile(name: &str, text: &str) {
    let path = format!("{}/{}", OUTPUT_DIR, name);
    fs::create_dir_all(&path).expect("Unable to create output directory");
    let uuid = Uuid::new_v4();
    let filename = format!("{}/log-{}.json", &path, &uuid.simple());
    let data = text.as_bytes();
    let mut file = File::create(&filename).expect(&format!("Unable to create file {}", &filename));
    file.write_all(data).expect("Write error");
    file.sync_data().expect("Sync data error");
}


pub fn round(val: f64, p: i32) -> f64 {
    let f = 10.0_f64.powi(p);
    ((val * f).round())/f
}
