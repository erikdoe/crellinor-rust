use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use serde_json;
use serde_derive::*;
use serde_json::{json, to_string_pretty};
use uuid::Uuid;
use crate::params::Params;
use crate::world::World;
use crate::random::RNG;


#[derive(Serialize, Deserialize, Debug)]
struct Worldfile {
    pub params: Params,
    pub seed: [u32; 4],
}

impl Worldfile {
    pub fn from_str(s: &str) -> Result<Worldfile, &'static str> {
        let wf: Worldfile = serde_json::from_str(s).expect("Failed to parse JSON");
        Ok(wf)
    }

    pub fn from_file(filename: &str) -> Result<Worldfile, &'static str> {
        let mut file = File::open(filename).expect(&format!("Failed to open file {}", filename));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Failed to read file {}", filename));
        Worldfile::from_str(&contents)
    }
}

pub fn load_world(path: &str) -> World {
    let name = Path::new(path).file_stem().unwrap().to_str().unwrap();
    println!("Loading world from {}", path);
    let wf = Worldfile::from_file(path).expect("Failed to load worldfile");
    let mut w = World::new(name, wf.params);
    w.random = RNG::from_seed(&wf.seed);
    w
}


pub fn write_world_with_log(w: &World) {
    // We're writing more fields but the loader will ignore them
    let name = w.name.as_ref().expect("Can't write world without name");
    let id = Uuid::new_v4().simple().to_string();
    let json = json!({
        "params": w.params,
        "seed": w.random.seed(),
        "cycles": w.cycle,
        "status": ({ if w.num_creatures() > 1 { "ENDOK" } else { "ENDAB" } }),
        "id": id,
        "x-log": w.log,
    });
    write_worldfile(name, &id, &to_string_pretty(&json).unwrap());
}

const OUTPUT_DIR: &str = "output";

pub fn write_worldfile(name: &str, id: &str, text: &str) {
    let path = format!("{}/{}", OUTPUT_DIR, name);
    fs::create_dir_all(&path).expect("Unable to create output directory");
    let filename = format!("{}/log-{}.json", &path, id);
    let data = text.as_bytes();
    let mut file = File::create(&filename).expect(&format!("Unable to create file {}", &filename));
    file.write_all(data).expect("Write error");
    file.sync_data().expect("Sync data error");
}

