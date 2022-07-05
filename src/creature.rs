use std::cmp;
use crate::program::*;
use crate::params::Params;
use crate::random::RNG;
use crate::terrain::Terrain;
use crate::genetics;


pub struct Creature {
    pub program: Vec<Instr>,

    pub bcycle: u64,
    pub lastprocd: u64,

    pub bearing: u16,
    pub ep: u32,

    pub pc: usize,
    pub cc: u64,
}


impl Creature {
    pub fn new(program: Vec<Instr>) -> Creature {
        Creature {
            program,
            bcycle: 0,
            lastprocd: 0,
            bearing: 0,
            ep: 0,
            pc: 0,
            cc: 0,
        }
    }

    // methods that change attributes/fields

    pub fn inc_pc(&mut self) {
        self.pc = (self.pc + 1) % self.program.len();
    }

    pub fn inc_pc_by(&mut self, n: usize) {
        self.pc = (self.pc + n) % self.program.len();
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

    pub fn pp_program(&self, rsize: usize) -> Vec<String> {
        let mut out = Vec::new();
        let mut i = 0;
        for _ in 0..(self.program.len() / rsize) {
            let mut ring = String::new();
            for _ in 0..rsize {
                ring.push_str(&format!("{:?} ", self.program[i]));
                i += 1;
            }
            out.push(ring);
        }
        out
    }

    pub fn program_as_string(&self) -> String {
        format!("{:5}  {:?}", self.age(), self.program)
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
        self.inc_pc();
        self.exec_instr(instr, ctx);
    }

    // dispatch table

    fn exec_instr(&mut self, instr: Instr, ctx: &mut PContext) {
        match instr {
            Instr::TUR => self.exec_turn_right(),
            Instr::TUL => self.exec_turn_left(),
            Instr::MOV => self.exec_move(ctx),
            Instr::EAT => self.exec_eat(ctx),
            Instr::JMP => self.exec_jump_abs(ctx),
            Instr::JRE => self.exec_jump_rel(ctx),
            Instr::BFH => self.exec_branch_food_here(ctx),
            Instr::BFA => self.exec_branch_food_ahead(ctx),
            Instr::NOP => self.exec_nop(),
        }
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

    fn exec_jump_rel(&mut self, ctx: &PContext) {
        self.inc_pc_by(ctx.params.ring_len - 1);
    }

    fn exec_jump_abs(&mut self, ctx: &PContext) {
        let p = self.pc % ctx.params.ring_len;
        if p > 0 {
            self.inc_pc_by(ctx.params.ring_len - p);
        }
    }

    // check and branch

    fn exec_branch_food_here(&mut self, ctx: &mut PContext) {
        if ctx.terrain.plant_at(ctx.pos).is_some() {
            self.exec_jump_abs(ctx);
        }
    }

    fn exec_branch_food_ahead(&mut self, ctx: &mut PContext) {
        let mut fpos = ctx.pos;
        for _ in 0..ctx.params.view_distance {
            fpos = ctx.terrain.pos_ahead(fpos, self.bearing);
            if ctx.terrain.plant_at(fpos).is_some() {
                self.exec_jump_abs(ctx);
                return;
            }
        }
    }

    // miscellaneous instructions

    fn exec_nop(&mut self) {}


    // mating

    fn try_mate(&mut self, partner_pos: (u32, u32), offspring_pos: (u32, u32), ctx: &mut PContext) {
        let mut result: Option<Creature> = None;
        if let Some(other) = ctx.terrain.creature_at(partner_pos) {
            if self.can_mate(other, ctx) {
                result = Some(self.mate(other, ctx.params, ctx.random, ctx.world_cycle));
            }
        }
        if let Some(offspring) = result {
            ctx.terrain.set_creature_at(Some(offspring), offspring_pos);
        }
    }

    fn can_mate(&self, other: &Creature, ctx: &PContext) -> bool {
        self.ep > ctx.params.min_mating_ep &&
            other.age() > (ctx.params.creature_start_ep + ctx.params.eat_ep) as u64
    }

    fn mate(&mut self, other: &Creature, params: &Params, random: &mut RNG, world_cycle: u64) -> Creature {
        let program = genetics::cut_n_splice_crossover(&self.program, &other.program, random);
        let mut offspring = Creature::new(program);
        offspring.bcycle = world_cycle;
        offspring.ep += params.creature_start_ep;
        self.ep -= params.creature_start_ep;
        return offspring;
    }
}


// -- Processing context structure

pub struct PContext<'a> {
    params: &'a Params,
    random: &'a mut RNG,
    terrain: &'a mut Terrain,
    world_cycle: u64,
    pos: (u32, u32),
}

impl<'a> PContext<'a> {
    pub fn new(params: &'a Params, random: &'a mut RNG, terrain: &'a mut Terrain,
               world_cycle: u64, pos: (u32, u32)) -> PContext<'a> {
        PContext {
            params,
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
    fn pp_program() {
        let c = Creature::new(vec![MOV, TUL, TUR, EAT, NOP, BFA]);

        let out = c.pp_program(3);

        println!("** {:?}", out);
    }

    #[test]
    fn age_when_not_processed_yet() {
        let mut c = Creature::new(vec![MOV, TUL, TUR, EAT, NOP, BFA]);
        c.bcycle = 512;

        assert_eq!(0, c.age())
    }
}