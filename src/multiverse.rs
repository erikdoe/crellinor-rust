use std::cmp;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32};
use core::sync::atomic::{Ordering};
use std::time::Instant;
use std::thread;
use maplit::*;
use crate::program::Instr::*;
use crate::params::Params;
use crate::random::RNG;
use crate::world::World;
use crate::utils::round;


const NUM_SIMS: u32 = 100;
const NUM_THREADS: u32 = 6;


pub fn run() {
    run_multiverse(make_world);
}


fn run_multiverse(worldfn: fn() -> World) {
    let mut handles = Vec::new();
    let counter = Arc::new(AtomicU32::new(0));

    for tnum in 0..NUM_THREADS {
        let thread_counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            loop {
                let snum = thread_counter.fetch_add(1, Ordering::Relaxed);
                if snum >= NUM_SIMS {
                    break;
                }
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
    let plant_start_ep = rng.choose(&[800, 1600, 2400]);
    let plant_prob = round(target_pop_size as f64 / plant_start_ep as f64, 3);

    let eat_ep = 800;

    let creature_start_ep = rng.choose(&[1000, 2000]);

    let params = Params {
        world_end: 500_000,
        log_interval: 2_500,

        world_size: 200,
        start_pop_size: 400,
        start_plant_count: 400,

        plant_start_ep,
        plant_prob,

        creature_max_age: 30_000,
        creature_start_ep,
        creature_max_ep: creature_start_ep * 3,

        eat_ep,
        min_mating_ep: creature_start_ep * 2,
        view_distance: 8,

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
