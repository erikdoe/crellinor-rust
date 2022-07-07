use core::cmp;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::path::Path;
use serde_json;
use serde_derive::*;
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


fn run_world(thread_num: u32, mut world: World) {
    println!("{:?}: Starting simulation.", thread_num);

    world.add_initial_plants_and_creatures();
    let start = Instant::now();
    world.do_cycles_until_end();
    let end = Instant::now();
    world.write_result();

    let duration = end.duration_since(start);
    let millis = cmp::max(1, duration.as_secs() * 1000 + duration.subsec_millis() as u64);
    let cpm = world.log.total_cycles / millis;
    println!("{:?}: Simulation ended after {} cycles.",
             thread_num, world.cycle);
    println!("{:?}: Processed {}×10\u{2076} program cycles in {}s ({} cycles/ms).",
             thread_num, world.log.total_cycles/1_000_000, millis/1000, cpm);
}

pub fn load_and_run(path: &str) {
    run_world(0, load_world(path));
}
