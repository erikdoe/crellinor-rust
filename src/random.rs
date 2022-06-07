use rand::prelude::*;
use rand_xorshift::XorShiftRng;


pub struct RNG {
    seed: [u32; 4],
    system_rng: XorShiftRng,
    stubbed_seq: Option<Vec<u32>>,
}


impl RNG {
    pub fn new() -> RNG {
        let mut rng = thread_rng();
        RNG::from_seed(&[rng.next_u32(), rng.next_u32(), rng.next_u32(), rng.next_u32()])
    }

    pub fn from_seed(seed: &[u32; 4]) -> RNG {
        let mut as_u8: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..4 {
            let mut s = seed[i];
            for j in 0..4 {
                as_u8[i * 4 + j] = (s & 0xFF) as u8;
                s >>= 8;
            }
        }
        RNG {
            seed: seed.clone(),
            system_rng: XorShiftRng::from_seed(as_u8),
            stubbed_seq: None,
        }
    }

    pub fn seed(&self) -> &[u32; 4] {
        &self.seed
    }

    pub fn next_u32(&mut self, ceiling: u32) -> u32 {
        if let Some(val) = self.get_next_value() {
            if val > ceiling {
                panic!("stubbed value is greater than ceiling; found {}", val);
            }
            return val;
        }
        self.system_rng.gen_range(0..ceiling)
    }

    pub fn next_usize(&mut self, ceiling: usize) -> usize {
        self.next_u32(ceiling as u32) as usize
    }

    pub fn next_f64(&mut self) -> f64 {
        // TODO: can't set these
        self.system_rng.gen::<f64>()
    }

    pub fn choose<T>(&mut self, values: &[T]) -> T where T: Clone {
        // TODO: can't set these
        values[self.system_rng.gen_range(0..values.len())].clone()
    }

    pub fn set_next_values(&mut self, values: &[u32]) {
        let mut seq = Vec::from(values);
        seq.reverse();
        self.stubbed_seq = Some(seq);
    }

    fn get_next_value(&mut self) -> Option<u32> {
        if let Some(ref mut seq) = self.stubbed_seq {
            return seq.pop();
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_sequence() {
        let mut rng = RNG::new();
        rng.set_next_values(&[1, 2, 3]);
        assert_eq!(1, rng.next_u32(10));
        assert_eq!(2, rng.next_u32(10));
        assert_eq!(3, rng.next_u32(10));
    }

    #[test]
    fn continues_when_stubbed_sequence_is_consumed() {
        let mut rng = RNG::new();
        rng.set_next_values(&[8]);
        assert_eq!(8, rng.next_u32(10));
        assert_eq!(true, rng.next_u32(10) < 10);
    }

    #[test]
    #[should_panic]
    fn panics_when_stubbed_value_is_too_large() {
        let mut rng = RNG::new();
        rng.set_next_values(&[8]);
        assert_eq!(8, rng.next_u32(3));
    }


    #[test]
    fn same_seed_should_result_in_same_sequence() {
        let seed = &[1, 2, 3, 4];
        let mut rng0 = RNG::from_seed(seed);
        let mut rng1 = RNG::from_seed(seed);

        for _ in 0..100 {
            assert_eq!(rng0.next_u32(100_000), rng1.next_u32(100_000));
        }
    }

    #[test]
    fn different_seed_should_result_in_different_sequence() {
        let mut rng0 = RNG::from_seed(&[1, 2, 3, 4]);
        let mut rng1 = RNG::from_seed(&[4, 3, 2, 1]);

        let mut found_difference = false;
        for _ in 0..100 {
            if rng0.next_u32(100_000) != rng1.next_u32(100_000) {
                found_difference = true;
                break;
            }
        }
        assert_eq!(true, found_difference);
    }
}
