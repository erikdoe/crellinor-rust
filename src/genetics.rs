use crate::program::Instr;
use crate::random::RNG;


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

pub fn mutation<'a>(p: &mut Vec<Instr>, instr_list: Vec<&Instr>, rng:&'a mut RNG) {
    let mpt = rng.next_usize(p.len());
    let instr = instr_list[rng.next_usize(instr_list.len())].clone();
    p[mpt] = instr;
}


#[cfg(test)]
mod tests {
    use crate::params::Params;
    use super::*;
    use crate::program::Instr::*;

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

    #[test]
    fn mutation_at_point() {
        let mut rng = RNG::new();
        let params = Params::for_testing();
        rng.set_next_values(&[2 /* mpt */, 2 /* instr */]);
        let mut p = vec![MOV, EAT, NOP, NOP];

        mutation(&mut p, params.instr_list(),&mut rng);

        assert_eq!([MOV, EAT, TUL, NOP], p.as_slice());

    }
}
