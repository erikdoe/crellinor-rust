use std::cmp;
use crate::program::*;
use crate::params::Params;
use crate::random::RNG;
use crate::terrain::Terrain;
use crate::genetics;
use crate::log::Log;


pub struct Creature {
    pub program: Vec<Instr>,
    pub rsize: usize,

    pub bcycle: u64,
    pub lastprocd: u64,

    pub bearing: u16,
    pub ep: u32,

    pub pc: usize,
    pub cc: u64,
}


impl Creature {
    pub fn new(program: Vec<Instr>, params: &Params) -> Creature {
        Creature {
            program,
            rsize: params.ring_size,
            bcycle: 0,
            lastprocd: 0,
            bearing: 0,
            ep: params.creature_start_ep,
            pc: 0,
            cc: 0,
        }
    }

    // methods that change attributes/fields

    pub fn pc_reset(&mut self) {
        self.pc = 0;
    }

    pub fn pc_incr(&mut self) {
        let pc_ring = self.pc % self.rsize;
        let new_pc_ring = (pc_ring + 1) % self.rsize;
        self.pc = self.pc - pc_ring + new_pc_ring;
    }

    pub fn pc_incr_ring(&mut self) {
        let ring = self.pc / self.rsize;
        let new_ring = (ring + 1) % (self.program.len() / self.rsize);
        self.pc = new_ring * self.rsize;
    }

    pub fn current_instr(&self) -> Instr {
        self.program[self.pc].clone()
    }

    pub fn add_to_bearing(&mut self, delta: u16) {
        self.bearing = (self.bearing + delta) % 360
    }


    // calculated attributes

    pub fn age(&self) -> u64 {
        if self.lastprocd == 0 {
            return 0;
        }
        self.lastprocd - self.bcycle
    }

    pub fn pp_program(&self) -> String {
        let mut out = String::new();
        for i in 0..self.program.len() {
            out.push_str(&format!("{:?} ", self.program[i]));
            if (i + 1) % self.rsize == 0 {
                out.push_str("; ");
            }
        }
        out
    }

    // core processing loop

    pub fn do_cycle(&mut self, ctx: &mut PContext) -> (u32, u32) {
        self.fetch_instr_if_necessary(ctx);
        self.cc -= 1;
        self.exec_instr_when_ready(ctx);
        ctx.pos
    }

    fn fetch_instr_if_necessary(&mut self, ctx: &mut PContext) {
        if self.cc > 0 {
            return;
        }
        let instr = self.current_instr();
        self.cc = ctx.params.instr_cycles(&instr);
    }

    fn exec_instr_when_ready(&mut self, ctx: &mut PContext) {
        if self.cc > 0 {
            return;
        }
        let instr = self.current_instr();
        self.pc_incr();
        self.exec_instr(instr, ctx);
    }

    // dispatch table

    fn exec_instr(&mut self, instr: Instr, ctx: &mut PContext) {
        match instr {
            Instr::TUR => self.exec_turn_right(),
            Instr::TUL => self.exec_turn_left(),
            Instr::MOV => self.exec_move(ctx),
            Instr::EAT => self.exec_eat(ctx),
            Instr::JMP => self.exec_jump(ctx),
            Instr::JMZ => self.exec_jump_zero(ctx),
            Instr::BFH => self.exec_branch_food_here(ctx),
            Instr::BFA => self.exec_branch_food_ahead(ctx),
            Instr::NOP => self.exec_nop(),
        }
        ctx.stats.incr_instr_counter(instr);
    }


    // turning and moving

    fn exec_turn_right(&mut self) {
        self.add_to_bearing(90);
    }

    fn exec_turn_left(&mut self) {
        self.add_to_bearing(270);
    }

    fn exec_move(&mut self, ctx: &mut PContext) {
        let target_pos = ctx.terrain.pos_ahead(ctx.pos, self.bearing);
        if ctx.terrain.creature_at(target_pos).is_none() {
            ctx.pos = target_pos;
        } else if let Some(offspring_pos) = ctx.terrain.free_pos_near(ctx.pos) {
            self.try_mate(target_pos, offspring_pos, ctx);
        }
    }

    // eating

    fn exec_eat(&mut self, ctx: &mut PContext) {
        if let Some(mut plant) = ctx.terrain.take_plant_at(ctx.pos) {
            let ep_consumable = cmp::min(ctx.params.eat_ep, ctx.params.creature_max_ep - self.ep);
            if plant.ep < ep_consumable {
                self.ep += plant.ep;
            } else {
                self.ep += ep_consumable;
                plant.ep -= ep_consumable;
                ctx.terrain.set_plant_at(Some(plant), ctx.pos);
            }
        }
    }

    // jumps

    fn exec_jump(&mut self, _: &PContext) {
        self.pc_incr_ring()
    }

    fn exec_jump_zero(&mut self, _: &PContext) {
        self.pc_reset();
    }

    // check and branch

    fn exec_branch_food_here(&mut self, ctx: &mut PContext) {
        if ctx.terrain.plant_at(ctx.pos).is_some() {
            self.exec_jump(ctx);
        }
    }

    fn exec_branch_food_ahead(&mut self, ctx: &mut PContext) {
        let mut fpos = ctx.pos;
        for _ in 0..ctx.params.view_distance {
            fpos = ctx.terrain.pos_ahead(fpos, self.bearing);
            if ctx.terrain.plant_at(fpos).is_some() {
                self.exec_jump(ctx);
                return;
            }
        }
    }

    // miscellaneous instructions

    fn exec_nop(&mut self) {}


    // mating

    fn try_mate(&mut self, partner_pos: (u32, u32), offspring_pos: (u32, u32), ctx: &mut PContext) {
        if let Some(other) = ctx.terrain.creature_at(partner_pos) {
            if self.can_mate(other, ctx) {
                let offspring = self.mate(other, ctx.params, ctx.random, ctx.world_cycle);
                ctx.terrain.set_creature_at(Some(offspring), offspring_pos);
            }
        }
    }

    fn can_mate(&self, other: &Creature, ctx: &PContext) -> bool {
        self.ep > ctx.params.min_mating_ep &&
            other.age() > (ctx.params.creature_start_ep + ctx.params.eat_ep) as u64
    }

    fn mate(&mut self, other: &Creature, params: &Params, random: &mut RNG, world_cycle: u64) -> Creature {
        let program = genetics::cut_n_splice_crossover(&self.program, &other.program, random);
        let mut offspring = Creature::new(program, params);
        offspring.bcycle = world_cycle;
        offspring.bearing = random.choose(&[0, 90, 180, 270]);
        self.ep -= offspring.ep;
        return offspring;
    }
}


// -- Processing context structure

pub struct PContext<'a> {
    params: &'a Params,
    stats: &'a mut Log,
    random: &'a mut RNG,
    terrain: &'a mut Terrain,
    world_cycle: u64,
    pos: (u32, u32),
}

impl<'a> PContext<'a> {
    pub fn new(params: &'a Params, stats: &'a mut Log, random: &'a mut RNG, terrain: &'a mut Terrain,
               world_cycle: u64, pos: (u32, u32)) -> PContext<'a> {
        PContext {
            params,
            stats,
            random,
            terrain,
            world_cycle,
            pos,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::Instr::*;

    #[test]
    fn age_when_not_processed_yet() {
        let mut c = Creature::new(vec![NOP], &Params::for_testing());
        c.bcycle = 512;
        assert_eq!(0, c.age())
    }

    #[test]
    fn pc_stays_in_ring0() {
        let mut c = Creature::new(vec![NOP], &Params::for_testing());
        c.pc = 2;
        c.pc_incr();
        assert_eq!(0, c.pc)
    }

    #[test]
    fn pc_stays_in_ring1() {
        let mut c = Creature::new(vec![NOP], &Params::for_testing());
        c.pc = 5;
        c.pc_incr();
        c.pc_incr();
        assert_eq!(4, c.pc)
    }

    #[test]
    fn pc_moves_to_next_ring() {
        let mut c = Creature::new(vec![NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP], &Params::for_testing());
        c.pc = 4;
        c.pc_incr_ring();
        assert_eq!(6, c.pc)
    }

    #[test]
    fn pc_wraps_to_first_ring() {
        let mut c = Creature::new(vec![NOP, NOP, NOP, NOP, NOP, NOP], &Params::for_testing());
        c.pc = 4;
        c.pc_incr_ring();
        assert_eq!(0, c.pc)
    }
}
