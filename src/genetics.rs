use core::iter;
use crate::program::Instr;
use crate::random::RNG;
use crate::params::Params;


pub fn rand_program(params: &Params, rng: &mut RNG) -> Vec<Instr> {
    let instr_list = params.instr_list();
    iter::repeat_with(|| instr_list[rng.next_usize(instr_list.len())].clone())
        .take(params.ring_count * params.ring_size).collect()
}


pub fn single_point_crossover<'a>(p0: &Vec<Instr>, p1: &Vec<Instr>, rng: &'a mut RNG) -> Vec<Instr> {
    let (left, right) = [(p0, p1), (p1, p0)][rng.next_usize(2)];
    let xpt = rng.next_usize(p0.len());
    let mut result = left[..xpt].to_vec();
    result.extend_from_slice(&right[xpt..]);
    result
}


pub fn cut_n_splice_crossover<'a>(p0: &Vec<Instr>, p1: &Vec<Instr>, rng: &'a mut RNG) -> Vec<Instr> {
    let l = rng.next_usize(p1.len() - 1);
    let s = rng.next_usize(p1.len() - l);
    let d = rng.next_usize(p0.len() - l);
    let mut pc = p0.clone();
    pc.splice(d..d + l, p1[s..s + l].iter().cloned());
    pc
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::Instr::*;

    #[test]
    fn random_program_with_two_instructions() {
        let params = Params::new();
        let mut rng = RNG::new();
        rng.set_next_values(&[2, 3]);

        let prog = rand_program(&params, &mut rng);

        assert_eq!(params.instr_list()[2], &prog[0]);
        assert_eq!(params.instr_list()[3], &prog[1]);
    }

    #[test]
    fn random_programs_with_same_seed_are_identical() {
        let params = Params::new();
        let mut rng0 = RNG::from_seed(&[1092393775, 1536878131, 2147757716, 2050134695]);
        let mut rng1 = RNG::from_seed(&[1092393775, 1536878131, 2147757716, 2050134695]);

        let prog0 = rand_program(&params, &mut rng0);
        let prog1 = rand_program(&params, &mut rng1);

        assert_eq!(prog0, prog1);
    }


    #[test]
    fn single_point_crossover_with_p0_first() {
        let mut rng = RNG::new();
        rng.set_next_values(&[0, 2]);
        let p0 = vec![NOP, TUL, MOV];
        let p1 = vec![MOV, MOV, TUR];

        let pc = single_point_crossover(&p0, &p1, &mut rng);

        assert_eq!([NOP, TUL, TUR], pc.as_slice());
    }

    #[test]
    fn single_point_crossover_with_p1_first() {
        let mut rng = RNG::new();
        rng.set_next_values(&[1, 1]);
        let p0 = vec![TUL, TUL];
        let p1 = vec![TUR, TUR];

        let pc = single_point_crossover(&p0, &p1, &mut rng);

        assert_eq!([TUR, TUL], pc.as_slice());
    }

    #[test]
    fn cut_n_splice_crossover_with_move() {
        let mut rng = RNG::new();
        rng.set_next_values(&[2 /* len */, 1 /* source pos */, 2 /* dest pos */]);
        let p0 = vec![TUL, TUL, TUL, TUL, TUR];
        let p1 = vec![MOV, NOP, NOP, MOV, MOV];

        let pc = cut_n_splice_crossover(&p0, &p1, &mut rng);

        assert_eq!([TUL, TUL, NOP, NOP, TUR], pc.as_slice());
    }
}
