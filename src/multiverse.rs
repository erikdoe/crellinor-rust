use std::cmp;
use std::time::Instant;
use std::thread;
use maplit::*;
use crate::program::Instr::*;
use crate::params::Params;
use crate::random::RNG;
use crate::world::World;
use crate::utils::round;


const NUM_SIMS: u32 = 600;
const NUM_THREADS: u32 = 6;


pub fn run() {
    run_multiverse(make_world);
}


fn run_multiverse(worldfn: fn() -> World) {
    let mut handles = Vec::new();

    for tnum in 0..NUM_THREADS {
        let h = thread::spawn(move || {
            for snum in 0..(NUM_SIMS / NUM_THREADS) {
                run_world(tnum, snum, worldfn());
            }
        });
        handles.push(h);
    }

    while let Some(h) = handles.pop() {
        h.join().unwrap();
    }
}


fn run_world(thread_num: u32, sim_num: u32, mut world: World) {
    println!("{}: Starting simulation #{}.", thread_num, sim_num);

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


fn make_world() -> World {
    let mut rng = RNG::new();

    let target_pop_size = 400;
    let plant_start_ep = 800;
    let plant_prob = round(target_pop_size as f64 / plant_start_ep as f64, 3);

    let world_size = 200;
    let ep_per_lane = world_size * (10 + 5);

    let eat_ep = plant_start_ep; // can eat entire plant


    let params = Params {
        world_end: 1_000_000,
        log_interval: 1_000,

        world_size: 200,
        start_pop_size: target_pop_size,
        start_plant_count: target_pop_size * 2,

        plant_start_ep,
        plant_prob,

        creature_max_age: 50_000,
        creature_start_ep: ep_per_lane,
        creature_max_ep: 3 * ep_per_lane,

        eat_ep,
        min_mating_ep: ep_per_lane * 2,
        view_distance: 5,

        ring_count: rng.choose(&[2, 3, 4]),
        ring_size: rng.choose(&[3, 4, 5]),

        instructions: hashmap! {
            EAT => 10,
            MOV =>  5,
            TUR =>  3,
            TUL =>  3,
            NOP =>  1,
            JMP =>  1,
            JMZ =>  1,
            BFH =>  1,
            BFA =>  1,
        },
    };




    World::new("progsize", params)
}

