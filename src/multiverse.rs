use std::cmp;
use std::time::Instant;
use std::thread;
use maplit::*;
use crate::program::Instr::*;
use crate::params::Params;
use crate::random::RNG;
use crate::world::World;
use crate::utils::round;


const NUM_SIMS: u32 = 24000;
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
    let plant_start_ep = 1500;
    let creature_start_ep = 1500;

    let params = Params {
        world_end: 800_000,
        log_interval: 2000,

        world_size: 200,
        start_pop_size: 1000,
        start_plant_count: 8000,

        plant_start_ep,
        plant_prob: round(180.0 / plant_start_ep as f64, 3),
        plant_prob_end: round(60.0 / plant_start_ep as f64, 3),

        creature_max_age: rng.choose(&[60_000, 65_000, 70_000, 75_000]),
        creature_start_ep,
        creature_max_ep: 20_000,

        eat_ep: 750,
        min_mating_ep: creature_start_ep * 2 + 200,  // should be greater than 2*start
        view_distance: 8,

        ring_count: rng.choose(&[3, 4, 5, 6, 7]),
        ring_size: rng.choose(&[3, 4, 5, 6, 7]),

        instructions: hashmap! {
            EAT => 30,
            MOV => 20,
            TUR => 10,
            TUL => 10,
            NOP =>  1,
            JMP =>  2,
            JMZ =>  2,
            BFH =>  3,
            BFA =>  3,
        },
    };
    World::new("eat30", params)
}

