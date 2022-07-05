use std::f64::consts::PI;
use serde_json::*;

use crate::creature::Creature;
use crate::creature::PContext;
use crate::params::Params;
use crate::plant::Plant;
use crate::random::RNG;
use crate::terrain::Terrain;
use crate::log::Log;
use crate::genetics;
use crate::utils;


pub struct World {
    pub name: Option<String>,
    pub params: Params,
    pub random: RNG,
    pub terrain: Terrain,
    pub cycle: u64,
    pub log: Log,
}

impl World {
    pub fn new(name: &str, params: Params) -> World {
        let terrain = Terrain::with_size(params.world_size);
        World {
            name: Some(name.to_owned()),
            params,
            random: RNG::new(),
            terrain,
            cycle: 0,
            log: Log::new(),
        }
    }

    pub fn for_testing() -> World {
        let params = Params::for_testing();
        let terrain = Terrain::with_size(params.world_size);
        World {
            name: None,
            params,
            random: RNG::new(),
            terrain,
            cycle: 0,
            log: Log::new(),
        }
    }


    // adding creatures and plants

    pub fn add_creature(&mut self, mut c: Creature, pos: (u32, u32)) {
        if c.bcycle == 0 {
            c.bcycle = self.cycle;
        }
        c.ep = self.params.creature_start_ep;
        self.terrain.set_creature_at(Some(c), pos);
    }

    pub fn creature_at(&self, pos: (u32, u32)) -> Option<&Creature> {
        self.terrain.creature_at(pos)
    }

    pub fn num_creatures(&self) -> u32 {
        self.terrain.num_creatures()
    }

    pub fn add_plant(&mut self, mut p: Plant, pos: (u32, u32)) {
        p.ep = self.params.plant_start_ep;
        self.terrain.set_plant_at(Some(p), pos);
    }

    pub fn plant_at(&self, pos: (u32, u32)) -> Option<&Plant> {
        self.terrain.plant_at(pos)
    }


    // adding plants and creatures randomly

    pub fn add_initial_plants_and_creatures(&mut self) {
        for _ in 0..self.params.start_pop_size {
            self.add_random_creature()
        }
        for _ in 0..self.params.start_plant_count {
            let ep = self.params.plant_start_ep;
            self.add_random_plant(ep);
        }
    }

    fn add_random_creature(&mut self) -> () {
        let prog = genetics::rand_program(&self.params, &mut self.random);
        let mut creature = Creature::new(prog);
        creature.ep = self.params.creature_start_ep;
        if let Some(pos) = self.terrain.rand_free_pos(&mut self.random) {
            self.add_creature(creature, pos);
        }
    }

    fn add_random_plant(&mut self, start_ep: u32) {
        let mut plant = Plant::new();
        plant.ep = start_ep;
        let pos = self.terrain.rand_pos(&mut self.random);
        self.add_plant(plant, pos);
    }


    // running one cycle of the simulation

    pub fn do_one_cycle(&mut self) {
        self.inc_worldtime();
        self.grow_plants();
        self.process_all_creatures();
    }

    pub fn inc_worldtime(&mut self) {
        self.cycle += 1;
    }

    fn grow_plants(&mut self) {
        let p = self.plant_prob();
        if self.random.next_f64() < p {
            let ep = (self.params.plant_prob * (self.params.plant_start_ep as f64) / p) as u32;
            self.add_random_plant(ep);
        }
    }

    fn plant_prob(&self) -> f64 {
        let p = &self.params;
        let we = p.world_end as f64;
        let t = self.cycle as f64;
        let f = ((PI * t / we).cos() + 1.0) / 2.0;
        p.plant_prob_end + f * (p.plant_prob - p.plant_prob_end)
    }

    fn process_all_creatures(&mut self) {
        let cycle = self.cycle;
        let params = &self.params;
        let random = &mut self.random;
        let statistics = &mut self.log;
        self.terrain.do_with_creatures_mut(|terrain, creature, pos|
            {
                creature.lastprocd = cycle;
                creature.ep -= 1;
                if (creature.age() >= params.creature_max_age) || (creature.ep == 0) {
                    return None;
                }
                statistics.total_cycles += 1;
                let mut ctx = PContext::new(params, random, terrain, cycle, pos);
                return Some(creature.do_cycle(&mut ctx));
            });
    }


    // running multiple cycles of the simulation

    pub fn do_cycles(&mut self, num: u64) {
        for _ in 0..num {
            self.do_one_cycle();
        }
    }

    pub fn do_cycles_until_end(&mut self) {
        while self.num_creatures() > 1 && self.cycle < self.params.world_end {
            let log_period = self.params.log_interval;
            self.do_cycles(log_period);
            self.log.add_entry(self.cycle);
            let num_creatures = self.num_creatures();
            self.log.set_num_creatures(num_creatures);
            if self.cycle >= self.params.world_end {
                self.log.set_programs(self.terrain.all_creatures());
            }
        }
    }


    // writing the result

    pub fn write_result(&mut self) {
        let json = json!({
            "cycles": self.cycle,
            "params": self.params,
            "seed": self.random.seed(),
            "status": ({ if self.num_creatures() > 1 { "ENDOK" } else { "ENDAB" } }),
            "x-log": self.log,
        });

        if let Some(name) = &self.name {
            utils::write_logfile(&name, &to_string_pretty(&json).unwrap());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::Instr::*;

    #[test]
    fn run_all_programs() {
        let mut w = World::for_testing();
        w.params.set_instr_cycles(TUR, 2);
        w.params.set_instr_cycles(MOV, 2);
        w.params.creature_max_age = 5;
        w.add_creature(Creature::new(vec![TUR, TUR, TUR]), (1, 1));
        w.do_cycles(2);
        w.add_creature(Creature::new(vec![TUR, TUR, TUR]), (2, 2));
        w.add_creature(Creature::new(vec![TUR, MOV, MOV]), (3, 3));
        w.do_cycles(2);
        assert_eq!(180, w.creature_at((1, 1)).unwrap().bearing);
        assert_eq!(90, w.creature_at((2, 2)).unwrap().bearing);
        assert_eq!(90, w.creature_at((3, 3)).unwrap().bearing);
        w.do_cycles(2);
        assert_eq!(false, w.creature_at((1, 1)).is_some());
        assert_eq!(180, w.creature_at((2, 2)).unwrap().bearing);
        assert_eq!(90, w.creature_at((4, 3)).unwrap().bearing);
    }

    #[test]
    fn plant_reduction() {
        let mut w = World::for_testing();
        w.params.plant_prob = 0.2;
        w.params.plant_prob_end = 0.1;
        w.params.world_end = 2_000_000;

        assert_eq!(0.2, w.plant_prob());
        w.cycle = 1_000_000;
//        assert_eq!(0.15, w.plant_prob());  TODO: precision problem
        w.cycle = 2_000_000;
        assert_eq!(0.1, w.plant_prob());
    }
}
