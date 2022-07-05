use std::collections::HashMap;
use serde_derive::*;
use maplit::*;
use crate::program::Instr;
use crate::program::Instr::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub world_end: u64,
    pub log_interval: u64,

    pub world_size: u32,
    pub start_pop_size: u32,
    pub start_plant_count: u32,

    pub plant_start_ep: u32,
    pub plant_prob: f64,
    pub plant_prob_end: f64,

    pub creature_max_age: u64,
    pub creature_start_ep: u32,
    pub creature_max_ep: u32,

    pub eat_ep: u32,                // must be smaller than creature_max_ep
    pub min_mating_ep: u32,         // should be greater than 2 * creature_start_ep
    pub view_distance: u32,         // high performance impact

    pub ring_count: usize,
    pub ring_size: usize,

    pub instructions: HashMap<Instr, u64>,
}


impl Params {
    pub fn new() -> Params {
        Params {
            world_end: 3_000_000,
            log_interval: 100_000,

            world_size: 150,
            start_pop_size: 500,
            start_plant_count: 3000,

            plant_start_ep: 1000,
            plant_prob: 0.25,
            plant_prob_end: 0.25,

            creature_start_ep: 500,
            creature_max_ep: 5000,
            creature_max_age: 50_000,

            eat_ep: 200,
            min_mating_ep: 4000,
            view_distance: 4,

            ring_size: 3,
            ring_count: 2,

            instructions: Params::default_instr_map(),
        }
    }

    pub fn for_testing() -> Params {
        let mut params = Params::new();
        params.ring_size = 3;
        params.ring_count = 2;
        params
    }

    fn default_instr_map() -> HashMap<Instr, u64> {
        hashmap! {
            EAT => 10,
            MOV =>  5,
            TUR =>  3,
            TUL =>  3,
            NOP =>  1,
            JMP =>  1,
            JMZ =>  1,
            BFH =>  1,
            BFA =>  1,
        }
    }

    pub fn instr_list(&self) -> Vec<&Instr> {
        let mut instr_vec: Vec<&Instr> = self.instructions.keys().collect();
        instr_vec.sort();
        instr_vec
    }

    pub fn instr_cycles(&self, instr: &Instr) -> u64 {
        *self.instructions.get(instr).unwrap()
    }

    pub fn set_instr_cycles(&mut self, instr: Instr, cycles: u64) {
        self.instructions.insert(instr, cycles);
    }

}


