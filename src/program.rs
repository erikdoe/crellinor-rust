use std::iter;
use serde_derive::*;
use crate::random::RNG;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub enum Instr {
    NOP,
    TUR,
    TUL,
    MOV,
    EAT,
    JMP,
    JMZ,
    BFH,
    BFA,
}


pub fn rand_program(instr_list: Vec<&Instr>, size:usize, rng: &mut RNG) -> Vec<Instr> {
    iter::repeat_with(|| instr_list[rng.next_usize(instr_list.len())].clone())
        .take(size).collect()
}


pub fn base_strategy(size: usize, rng: &mut RNG) -> Vec<Instr> {
    let instr_list = &[ Instr:: MOV, Instr::EAT ];
    let mut idx = rng.next_usize(2);
    iter::repeat_with(|| { idx = (idx + 1) % 2; instr_list[idx] } )
        .take(size).collect()
}


#[cfg(test)]
mod tests {
    use crate::params::Params;
    use crate::program::Instr::*;
    use super::*;

    #[test]
    fn random_program_with_two_instructions() {
        let params = Params::new();
        let mut rng = RNG::new();
        rng.set_next_values(&[2, 3]);

        let prog = rand_program(params.instr_list(), 2, &mut rng);

        assert_eq!(params.instr_list()[2], &prog[0]);
        assert_eq!(params.instr_list()[3], &prog[1]);
    }

    #[test]
    fn random_programs_with_same_seed_are_identical() {
        let params = Params::new();
        let mut rng0 = RNG::from_seed(&[1092393775, 1536878131, 2147757716, 2050134695]);
        let mut rng1 = RNG::from_seed(&[1092393775, 1536878131, 2147757716, 2050134695]);

        let prog0 = rand_program(params.instr_list(), 20, &mut rng0);
        let prog1 = rand_program(params.instr_list(), 20, &mut rng1);

        assert_eq!(prog0, prog1);
    }

    #[test]
    fn base_strategy_alternates_eat_and_mov() {
        let mut rng = RNG::new();
        let prog = base_strategy(4, &mut rng);
        assert!(prog[0] == EAT || prog[0] == MOV);
        assert!((prog[1] == EAT || prog[1] == MOV) && prog[1] != prog[0]);
        assert!((prog[2] == EAT || prog[2] == MOV) && prog[2] != prog[1]);
        assert!((prog[3] == EAT || prog[3] == MOV) && prog[3] != prog[2]);
    }

    #[test]
    fn base_strategy_depends_on_rng() {
        let mut rng0 = RNG::new();
        rng0.set_next_values(&[0]);
        let prog0 = base_strategy(4, &mut rng0);

        let mut rng1 = RNG::new();
        rng1.set_next_values(&[1]);
        let prog1 = base_strategy(4, &mut rng1);

        assert_ne!(prog0[0], prog1[0]);
    }
}