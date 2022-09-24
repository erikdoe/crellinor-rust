use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;
use std::thread;
use maplit::*;
use crate::program::Instr::*;
use crate::params::Params;
use crate::random::RNG;
use crate::world::World;
use crate::utils::{round, square};


const NUM_SIMS: u32 = 4000;
const NUM_THREADS: u32 = 10;


fn make_world() -> World {
    let mut rng = RNG::new();

    // The world is square and world_size gives the length of the square.
    let world_size = 300;

    // How many creatures we want to live in the world. This is not a parameter but it's used in
    // calculations below. Having one creature per 100 locations has proven to be a good value.
    let target_pop_size = square(world_size) / 100;

    // Because of the simple [EAT MOV] strategy of the initial population some creatures will run
    // into each other, unable to move. These creatures will starve after creature_max_age cycles.
    // To avoid an energy surplus, which would lead to population size oscillations, we make the
    // start population slightly larger than the target population.
    let start_pop_size = target_pop_size * 5/4;

    // The amount of initial plants needed to start a "smooth" simulation depends on a number
    // of factors, including eat_ep and plant_start_ep. Too much or too little energy in the world
    // at the beginning leads to an initial phase with large oscillations of population size.
    let start_plant_count = 6000;

    // Each creature needs one EP per cycle. This means we can predetermine (roughly) the number of
    // creatures roaming the world. The number is roughly equal to the amount of EP put into the
    // system per cycle in most cases. (Exception have been observed!) That amount is, of course,
    // the probability that a plant grows multiplied with the plants' initial EP.
    let plant_start_ep = 3200;
    let plant_prob = round(target_pop_size as f64 / plant_start_ep as f64, 3);

    // Over the course of the simulation the probability will be reduced using a sigmoid function,
    // mapping [-10, +10] to [0, world_end]. The value for plant_start_ep is increased accordingly
    // to keep the same overall energy input.
    let plant_prob_end = plant_prob * 0.5;

    // The maximum EP needs to be enough to survive for a while without finding a plant.
    let creature_max_ep = 4000;

    // Creatures start with 1/2 of the maximum EP.
    let creature_start_ep = creature_max_ep * 1/2;

    // When creatures mate the initiating parent passes creature_start_ep onto the offspring.
    // Setting the min_mating_ep to 50% above the starting EP ensures the parent has at least 50%
    // starting EP left after mating.
    let min_mating_ep = creature_start_ep * 3/2;

    // Creatures don't necessarily consume the entire plant. When they can only eat a fraction that
    // should push them towards staying around to eat up the plant.
    let eat_ep = 800;

    // Empirically, a good value. Keeps successful creatures around for long enough to have a
    // number of chances to mate.
    let creature_max_age = rng.choose(&[80_000]);

    // Determines how far the creatures' BFA command can see.
    let view_distance = 6;


    let params = Params {
        world_end: 2_000_000,
        log_interval: 10_000,

        world_size,
        start_pop_size,
        start_plant_count,

        plant_start_ep,
        plant_prob,
        plant_prob_end,

        creature_max_age,
        creature_max_ep,
        creature_start_ep,
        min_mating_ep,
        eat_ep,
        view_distance,

        ring_count: rng.choose(&[2, 3]),
        ring_size: rng.choose(&[3, 4, 5, 6]),

        instructions: hashmap! {
            EAT => 25,
            MOV => 15,
            TUR =>  3,
            TUL =>  3,
            NOP =>  1,
            JMP =>  1,
            JMZ =>  1,
            BFH =>  1,
            BFA =>  1,
        },
    };

    World::new("ringstruct", params)
}


// running the worlds

pub fn run() {
    run_multiverse(make_world);
}


fn run_multiverse(worldfn: fn() -> World) {
    let mut handles = Vec::new();
    let counter = Arc::new(AtomicU32::new(0));

    for _ in 0..NUM_THREADS {
        let thread_counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            loop {
                let sim_num = thread_counter.fetch_add(1, Ordering::Relaxed);
                if sim_num >= NUM_SIMS {
                    break;
                }
                println!("Starting simulation #{}.", sim_num);
                let mut world = worldfn();
                world.run();
                println!("Simulation #{} ended after {} cycles.", sim_num, world.cycle);
            }
        });
        handles.push(h);
    }

    while let Some(h) = handles.pop() {
        h.join().unwrap();
    }
    println!("Done. Reached the end of all worlds.");
}

