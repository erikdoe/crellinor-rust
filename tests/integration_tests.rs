extern crate crellinor;

use crellinor::creature::Creature;
use crellinor::plant::Plant;
use crellinor::program::Instr::*;
use crellinor::world::World;
use crellinor::program::Instr;
use crellinor::params::Params;


// -- adding plants and creatures

#[test]
fn it_adds_creature() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![], &w.params), (1, 2));
    if let Some(bob) = w.creature_at((1, 2)) {
        assert_eq!(0, bob.bcycle);
    } else {
        panic!("no creature");
    }
}

#[test]
fn it_adds_plant() {
    let mut w = World::for_testing();
    w.add_plant(Plant::new(), (1, 2));
    assert_eq!(true, w.plant_at((1, 2)).is_some());
}


// -- instruction processing

#[test]
fn processing_increments_pc_and_updates_lastprocd() {
    let mut w = World::for_testing();
    // test is designed assuming NOP is 1 cycle
    assert_eq!(1, w.params.instr_cycles(&NOP));
    (0..8).for_each(|_| w.inc_worldtime());
    w.add_creature(Creature::new(vec![NOP, NOP], &w.params), (0, 0));
    w.do_one_cycle();
    let bob = w.creature_at((0, 0)).unwrap();
    assert_eq!(1, bob.pc);
    assert_eq!(9, bob.lastprocd);
}

#[test]
fn processing_restarts_ring_at_end() {
    let mut w = World::for_testing();
    // test is designed assuming NOP is 1 cycle
    assert_eq!(1, w.params.instr_cycles(&NOP));
    w.add_creature(Creature::new(vec![NOP, NOP, NOP, NOP, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(4);
    assert_eq!(1, w.creature_at((0, 0)).unwrap().pc);
}

#[test]
fn processing_pc_incremented_only_when_all_cycles_have_passed_for_instr() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![TUR, NOP, NOP, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(1);
    if let Some(bob) = w.creature_at((0, 0)) {
        assert_eq!(0, bob.pc);
        assert_ne!(0, bob.cc);
    }
    w.do_cycles(cycle_count(&w.params, &[TUR]) - 1);
    if let Some(bob) = w.creature_at((0, 0)) {
        assert_eq!(1, bob.pc);
        assert_eq!(0, bob.cc);
    }
}

#[test]
fn processing_each_creature_once_per_cycle() {
    // when a creature moves to a new loc that wasn't processed yet
    // its program shouldn't be run again on the new loc
    // TODO: this case can't occur anyway with current data structures
    let mut w = World::for_testing();
    let mut c = Creature::new(vec![MOV, NOP, TUL], &w.params);
    c.bearing = 90;
    w.add_creature(c, (1, 0));
    w.do_cycles(cycle_count(&w.params, &[MOV]));
    assert_eq!(1, w.creature_at((2, 0)).unwrap().pc);
}


// -- max age

#[test]
fn processing_removes_creature_when_it_reaches_max_age() {
    let mut w = World::for_testing();
    w.params.creature_max_age = 10;
    let c = Creature::new(vec![NOP, NOP, NOP, NOP, NOP, NOP], &w.params);
    w.add_creature(c, (1, 2));
    w.do_cycles(9);
    assert_eq!(true, w.creature_at((1, 2)).is_some());
    w.do_cycles(1);
    assert_eq!(false, w.creature_at((1, 2)).is_some());
}


// -- movement

#[test]
fn turn_right_adds_90deg_to_bearing() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUR], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(90, w.creature_at((1, 2)).unwrap().bearing);
}

#[test]
fn turn_left_subtracts_90deg_from_bearing_which_stays_0_360() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUL], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(270, w.creature_at((1, 2)).unwrap().bearing);
}

#[test]
fn turn_left_and_right_cancel_each_other() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUL, TUR], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(0, w.creature_at((1, 2)).unwrap().bearing);
}

#[test]
fn moves_north_when_bearing_is_0() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![MOV], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(true, w.creature_at((1, 2)).is_none());
    assert_eq!(true, w.creature_at((1, 1)).is_some());
}

#[test]
fn moves_east_when_bearing_is_90() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUR, MOV], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(true, w.creature_at((2, 2)).is_some());
}

#[test]
fn moves_south_when_bearing_is_180() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUR, TUR, MOV], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(true, w.creature_at((1, 3)).is_some());
}

#[test]
fn moves_west_when_bearing_is_270() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![TUR, TUR, MOV], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (1, 2));
    w.do_cycles(n);
    assert_eq!(true, w.creature_at((1, 3)).is_some());
}

#[test]
fn move_treats_terrain_as_a_torus() {
    let mut w = World::for_testing();
    let c = Creature::new(vec![MOV, TUL, MOV], &w.params);
    let n = cycle_count(&w.params, &c.program);
    w.add_creature(c, (0, 0));
    w.do_cycles(n);
    let max_xy = w.params.world_size - 1;
    assert_eq!(true, w.creature_at((max_xy, max_xy)).is_some());
}


// -- mating

#[test]
fn creatures_mate_when_one_bumps_into_another_and_conditions_are_met() {
    let mut w = World::for_testing();
    w.params.min_mating_ep = w.params.creature_start_ep;
    w.params.set_instr_cycles(EAT, 1);
    w.params.set_instr_cycles(MOV, 1);
    w.cycle = (w.params.creature_start_ep + 2 * w.params.eat_ep) as u64;

    w.add_plant(Plant::new(), (1, 1));
    w.add_creature(Creature::new(vec![EAT, MOV], &w.params), (1, 1));
    let mut partner = Creature::new(vec![NOP, NOP], &w.params);
    partner.bcycle = 1; // make partner look as if it has lived through enough cycles
    w.add_creature(partner, (1, 0));
    w.do_cycles(2);

    assert_eq!(3, w.num_creatures());
    // TODO: making assumptions about the offspring placement here
    assert_eq!(w.cycle, w.creature_at((2, 1)).unwrap().bcycle);
    assert_eq!(w.params.eat_ep - 2, w.creature_at((1, 1)).unwrap().ep);
    assert_eq!(w.params.creature_start_ep - 2, w.creature_at((1, 0)).unwrap().ep);
    assert_eq!(w.params.creature_start_ep - 1, w.creature_at((2, 1)).unwrap().ep);
}

#[test]
fn creatures_dont_mate_if_parent0_does_not_have_enough_ep() {
    let mut w = World::for_testing();
    w.params.min_mating_ep = w.params.creature_start_ep;
    w.params.set_instr_cycles(MOV, 1);
    w.cycle = (w.params.creature_start_ep + 2 * w.params.eat_ep) as u64;

    w.add_creature(Creature::new(vec![NOP, MOV], &w.params), (1, 1));
    let mut partner = Creature::new(vec![NOP, NOP], &w.params);
    partner.bcycle = 1; // make partner look as if it has lived through enough cycles
    w.add_creature(partner, (1, 0));
    w.do_cycles(2);

    assert_eq!(2, w.num_creatures());
}

#[test]
fn creatures_dont_mate_if_parent1_is_not_mature() {
    let mut w = World::for_testing();
    w.params.min_mating_ep = w.params.creature_start_ep;
    w.params.set_instr_cycles(EAT, 1);
    w.params.set_instr_cycles(MOV, 1);

    w.add_plant(Plant::new(), (1, 1));
    w.add_creature(Creature::new(vec![EAT, MOV], &w.params), (1, 1));
    let partner = Creature::new(vec![NOP, NOP], &w.params);
    w.add_creature(partner, (1, 0));
    w.do_cycles(2);

    assert_eq!(2, w.num_creatures());
}


// -- eating and energy points

#[test]
fn processing_takes_one_ep_per_cycle() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![NOP, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(2);
    let bob = w.creature_at((0, 0)).unwrap();
    assert_eq!(w.params.creature_start_ep - 2, bob.ep);
}

#[test]
fn processing_removes_creature_when_no_ep_left() {
    let mut w = World::for_testing();
    w.params.creature_start_ep = 1;
    w.add_creature(Creature::new(vec![NOP], &w.params), (0, 0));
    w.do_cycles(2);
    assert_eq!(false, w.creature_at((0, 0)).is_some())
}

#[test]
fn eat_transfers_ep_from_plant_to_creature() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![EAT], &w.params), (0, 0));
    w.add_plant(Plant::new(), (0, 0));
    let n = cycle_count(&w.params, &[EAT]);
    w.do_cycles(n);
    let expected_ep = w.params.creature_start_ep + w.params.eat_ep - n as u32;
    assert_eq!(expected_ep, w.creature_at((0, 0)).unwrap().ep);
    let expected_ep = w.params.plant_start_ep - w.params.eat_ep ;
    assert_eq!(expected_ep, w.plant_at((0, 0)).unwrap().ep);
}

#[test]
fn eat_removes_plant_when_no_ep_left() {
    let mut w = World::for_testing();
    w.params.plant_start_ep = 12;
    w.add_creature(Creature::new(vec![EAT], &w.params), (0, 0));
    w.add_plant(Plant::new(), (0, 0));
    let n = cycle_count(&w.params, &[EAT]);
    w.do_cycles(n);
    let expected_ep = w.params.creature_start_ep + 12 - n as u32;
    assert_eq!(expected_ep, w.creature_at((0, 0)).unwrap().ep);
    assert_eq!(false, w.plant_at((0, 0)).is_some());
}

#[test]
fn eat_does_not_exceed_creatures_map_ep() {
    let mut w = World::for_testing();
    w.params.creature_start_ep = w.params.creature_max_ep - 10;
    w.add_creature(Creature::new(vec![EAT], &w.params), (0, 0));
    w.add_plant(Plant::new(), (0, 0));
    let n = cycle_count(&w.params, &[EAT]);
    w.do_cycles(n);
    let expected_ep = w.params.creature_max_ep;
    assert_eq!(expected_ep, w.creature_at((0, 0)).unwrap().ep);
    let expected_ep = w.params.plant_start_ep - 10 - n as u32;
    assert_eq!(expected_ep, w.plant_at((0, 0)).unwrap().ep);
}


// -- jumps

#[test]
fn jump_jumps_to_first_position_in_next_ring() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![NOP, JMP, EAT, MOV, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[NOP, JMP]));
    assert_eq!(MOV, w.creature_at((0, 0)).unwrap().current_instr());
}

#[test]
fn jump_jumps_to_first_position_in_next_ring_also_when_last_in_ring() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![NOP, NOP, JMP, MOV, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[NOP, NOP, JMP]));
    assert_eq!(MOV, w.creature_at((0, 0)).unwrap().current_instr());
}


#[test]
fn jump_zero_jumps_to_beginning_of_ring0() {
    let mut w = World::for_testing();
    w.params.ring_size = 3;
    w.params.ring_count = 2;
    w.add_creature(Creature::new(vec![NOP, JMP, EAT, TUL, JMZ, TUR], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[NOP, JMP, TUL, JMZ]));
    assert_eq!(NOP, w.creature_at((0, 0)).unwrap().current_instr());
}


// -- combined check and branch

#[test]
fn branch_food_here_jumps_when_food_is_here() {
    let mut w = World::for_testing();
    w.add_plant(Plant::new(), (0, 0));
    w.add_creature(Creature::new(vec![NOP, BFH, NOP, EAT, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[NOP, BFH]));
    assert_eq!(EAT, w.creature_at((0, 0)).unwrap().current_instr());
}

#[test]
fn branch_food_here_continues_when_no_food_is_here() {
    let mut w = World::for_testing();
    w.add_creature(Creature::new(vec![BFH, NOP, TUL, TUR, EAT], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[BFH]));
    assert_eq!(NOP, w.creature_at((0, 0)).unwrap().current_instr());
}

#[test]
fn branch_food_ahead_jumps_when_plant_is_in_view_distance() {
    let mut w = World::for_testing();
    w.params.ring_size = 3;
    w.params.view_distance = 4;
    w.add_plant(Plant::new(), (4, 0));
    w.add_creature(Creature::new(vec![TUR, BFA, MOV, EAT, NOP, NOP], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[TUR, BFA]));
    assert_eq!(EAT, w.creature_at((0, 0)).unwrap().current_instr());

}

#[test]
fn branch_food_ahead_continues_when_no_plant_is_in_view_distance() {
    let mut w = World::for_testing();
    w.params.view_distance = 4;
    w.add_plant(Plant::new(), (5, 0));
    w.add_creature(Creature::new(vec![TUR, BFA, MOV, NOP, NOP, EAT], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[TUR, BFA]));
    assert_eq!(MOV, w.creature_at((0, 0)).unwrap().current_instr());

}

#[test]
fn branch_food_ahead_continues_when_plant_is_here() {
    let mut w = World::for_testing();
    w.params.view_distance = 4;
    w.add_plant(Plant::new(), (0, 0));
    w.add_creature(Creature::new(vec![TUR, BFA, MOV, NOP, NOP, EAT], &w.params), (0, 0));
    w.do_cycles(cycle_count(&w.params, &[TUR, BFA]));
    assert_eq!(MOV, w.creature_at((0, 0)).unwrap().current_instr());

}

// helper functions

pub fn cycle_count(params: &Params, prog: &[Instr]) -> u64 {
    prog.iter().map(|instr| params.instr_cycles(instr)).sum()
}

