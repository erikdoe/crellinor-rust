use crate::creature::Creature;
use crate::plant::Plant;
use crate::random::RNG;


pub struct Terrain {
    size: u32,
    creatures: Vec<Option<Creature>>,
    plants: Vec<Option<Plant>>,
    occupied: Vec<usize>,
}


impl Terrain {
    pub fn with_size(size: u32) -> Terrain {
        Terrain {
            size,
            creatures: Terrain::make_nones(size * size),
            plants: Terrain::make_nones(size * size),
            occupied: Vec::with_capacity(1000), // TODO: initial pop * 2
        }
    }

    fn make_nones<T>(n: u32) -> Vec<Option<T>> {
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            result.push(None);
        }
        result
    }


    // calculated attributes

    pub fn num_creatures(&self) -> u32 {
        self.occupied.len() as u32
    }

    pub fn all_creatures(&self) -> Vec<&Creature> {
        self.occupied
            .iter()
            .filter_map(|&i| self.creature_at(self.idx_to_pos(i as usize)))
            .collect()
    }

    pub fn all_creatures_with_pos(&self) -> Vec<((u32, u32), &Creature)> {
        self.occupied
            .iter()
            .filter_map(|&i| {
                let p = self.idx_to_pos(i as usize);
                self.creature_at(p).map(|c| (p, c))
            })
            .collect()
    }

    pub fn all_plants_with_pos(&self) -> Vec<((u32, u32), &Plant)> {
        let mut out = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let i = self.pos_to_idx((x, y));
                if self.plants[i].is_some() {
                    out.push(((x, y), self.plants[i].as_ref().unwrap()));
                }
            }
        }
        out
    }

    // translating positions and indices (private)

    fn idx_to_pos(&self, idx: usize) -> (u32, u32) {
        let i = idx as u32;
        ((i % self.size), (i / self.size))
    }

    fn pos_to_idx(&self, pos: (u32, u32)) -> usize {
        (pos.1 * self.size + pos.0) as usize
    }


    // calculating positions relative to each other

    pub fn pos_ahead(&self, pos: (u32, u32), bearing: u16) -> (u32, u32) {
        self.add_to_pos(pos, Terrain::dpos(bearing, 1))
    }

    pub fn pos_ahead_fast(&self, pos: (u32, u32), bearing: u16) -> (u32, u32) {
        let s = self.size;
        match bearing {
            0 => (pos.0, (pos.1 + s - 1) % s),
            90 => ((pos.0 + 1) % s, pos.1),
            180 => (pos.0, (pos.1 + 1) % s),
            270 => ((pos.0 + s - 1) % s, pos.1),
            _ => panic!("*** invalid bearing; found {}", bearing),
        }
    }

    pub fn beam_ahead(&self, pos: (u32, u32), bearing: u16, length: u32) -> Vec<(u32, u32)> {
        let mut beam = Vec::new();
        for d in 1..=length {
            beam.push(self.add_to_pos(pos, Terrain::dpos(bearing, d as i32)));
        }
        beam
    }

    pub fn free_pos_near(&self, pos: (u32, u32)) -> Option<(u32, u32)> {
        for bearing in [0, 90, 180, 270].iter() {
            let p = self.pos_ahead(pos, *bearing as u16);
            if self.creature_at(p).is_none() {
                return Some(p);
            }
        }
        None
    }

    pub fn free_pos_near_fast(&self, pos: (u32, u32)) -> Option<(u32, u32)> {
        let mut bearing = 0;
        for _ in 0..4 {
            let p = self.pos_ahead(pos, bearing);
            if self.creature_at(p).is_none() {
                return Some(p);
            }
            bearing += 90;
        }
        None
    }

    fn add_to_pos(&self, pos: (u32, u32), d: (i32, i32)) -> (u32, u32) {
        // note: we're converting the values on pos into i32 for the calculation!
        // adding s because % is remainder, and not modulo
        let s: i32 = self.size as i32;
        (((pos.0 as i32 + d.0 + s) % s) as u32,
         ((pos.1 as i32 + d.1 + s) % s) as u32)
    }

    fn dpos(bearing: u16, n: i32) -> (i32, i32) {
        match bearing {
            0 => (0, -1 * n),
            90 => (n, 0),
            180 => (0, n),
            270 => (-1 * n, 0),
            _ => panic!("*** invalid bearing; found {}", bearing),
        }
    }

    pub fn rand_pos(&self, rng: &mut RNG) -> (u32, u32) {
        // TODO: untested
        (rng.next_u32(self.size), rng.next_u32(self.size))
    }

    pub fn rand_free_pos(&self, rng: &mut RNG) -> Option<(u32, u32)> {
        // TODO: untested
        for _ in 0..20 {
            let p = self.rand_pos(rng);
            if self.creature_at(p).is_none() {
                return Some(p);
            }
        }
        None
    }

    // adding/removing creatures and plants

    pub fn set_creature_at(&mut self, c: Option<Creature>, pos: (u32, u32)) {
        let idx = self.pos_to_idx(pos);
        self.creatures[idx] = c;
        self.occupied.push(idx); // TODO: clearly wrong when c == None
    }

    pub fn creature_at(&self, pos: (u32, u32)) -> Option<&Creature> {
        self.creatures[self.pos_to_idx(pos)].as_ref()
    }

    pub fn set_plant_at(&mut self, p: Option<Plant>, pos: (u32, u32)) {
        let idx = self.pos_to_idx(pos);
        self.plants[idx] = p;
    }

    pub fn plant_at(&self, pos: (u32, u32)) -> Option<&Plant> {
        self.plants[self.pos_to_idx(pos)].as_ref()
    }

    pub fn take_plant_at(&mut self, pos: (u32, u32)) -> Option<Plant> {
        let idx = self.pos_to_idx(pos);
        if self.plants[idx].is_some() {
            return self.plants[idx].take();
        }
        return None;
    }


    // iterating over all creatures

    pub fn do_with_creatures_mut<F>(&mut self, mut func: F)
        where F: FnMut(&mut Terrain, &mut Creature, (u32, u32)) -> Option<(u32, u32)> {
        let mut j: usize = 0;
        while j < self.occupied.len() {
            let idx = self.occupied[j];
            let mut creature = self.creatures[idx].take().unwrap();
            let pos_before = self.idx_to_pos(idx);
            if let Some(pos_after) = func(self, &mut creature, pos_before) {
                let new_idx = self.pos_to_idx(pos_after);
                self.creatures[new_idx] = Some(creature);
                self.occupied[j] = new_idx;
                j += 1;
            } else {
                self.occupied.swap_remove(j);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::Instr::*;

    #[test]
    fn pos_to_idx() {
        let t = Terrain::with_size(10);
        assert_eq!(21, t.pos_to_idx((1, 2)));
    }

    #[test]
    fn idx_to_pos() {
        let t = Terrain::with_size(10);
        assert_eq!((1, 2), t.idx_to_pos(21));
    }

    #[test]
    fn pos_ahead_east() {
        let t = &Terrain::with_size(10);
        assert_eq!((2, 1), t.pos_ahead((1, 1), 90));
    }

    #[test]
    fn pos_head_north_with_wrapping() {
        let w = &Terrain::with_size(10);
        assert_eq!((1, 9), w.pos_ahead((1, 0), 0));
    }

    #[test]
    fn beam_ahead_north_with_wrapping() {
        let w = &Terrain::with_size(10);
        let beam = w.beam_ahead((1, 2), 0, 3);
        assert_eq!(3, beam.len());
        assert_eq!((1, 1), beam[0]);
        assert_eq!((1, 0), beam[1]);
        assert_eq!((1, 9), beam[2]);
    }

    #[test]
    fn free_pos_all_clear() {
        let w = &Terrain::with_size(10);
        assert_eq!(Some((1, 0)), w.free_pos_near((1, 1)));
    }

    #[test]
    fn free_pos_north_occupied() {
        let mut t = Terrain::with_size(10);
        t.set_creature_at(Some(Creature::new(vec![NOP], 3)), (1, 0));
        assert_eq!(Some((2, 1)), t.free_pos_near((1, 1)));
    }

    #[test]
    fn free_pos_all_occupied() {
        let mut w = Terrain::with_size(10);
        w.set_creature_at(Some(Creature::new(vec![NOP], 3)), (1, 0));
        w.set_creature_at(Some(Creature::new(vec![NOP], 3)), (2, 1));
        w.set_creature_at(Some(Creature::new(vec![NOP], 3)), (1, 2));
        w.set_creature_at(Some(Creature::new(vec![NOP], 3)), (0, 1));
        assert_eq!(None, w.free_pos_near((1, 1)));
    }

    #[test]
    fn adding_to_occupied_index_list() {
        let mut t = Terrain::with_size(10);
        t.set_creature_at(Some(Creature::new(vec![NOP], 3)), (4, 7));
        t.set_creature_at(Some(Creature::new(vec![MOV], 3)), (3, 3));
        let list = &t.occupied;
        assert_eq!(2, list.len());
        // slightly white-box; theoretically the index could be at list[1]
        assert_eq!((4, 7), t.idx_to_pos(list[0]));
    }

    #[test]
    fn all_creatures() {
        let mut t = Terrain::with_size(10);
        t.set_creature_at(Some(Creature::new(vec![NOP], 3)), (4, 7));
        t.set_creature_at(Some(Creature::new(vec![MOV], 3)), (3, 3));

        let list = t.all_creatures();

        assert_eq!(2, list.len());
        assert_eq!(true, list.iter().find(|&c| c.program == vec![NOP]).is_some());
        assert_eq!(true, list.iter().find(|&c| c.program == vec![MOV]).is_some());
    }

    #[test]
    fn all_creatures_with_pos() {
        let mut t = Terrain::with_size(10);
        t.set_creature_at(Some(Creature::new(vec![NOP], 3)), (4, 7));

        let list = t.all_creatures_with_pos();

        assert_eq!(1, list.len());
        assert_eq!((4, 7), list[0].0);
        assert_eq!(vec![NOP], list[0].1.program);
    }

    #[test]
    fn all_plants_with_pos() {
        let mut t = Terrain::with_size(10);
        t.set_plant_at(Some(Plant::with_ep(72)), (7, 2));
        t.set_plant_at(Some(Plant::with_ep(14)), (1, 4));

        let mut list = t.all_plants_with_pos();

        assert_eq!(2, list.len());
        list.sort_by(|a, b| a.1.ep.cmp(&b.1.ep) );

        assert_eq!(14, list[0].1.ep);
        assert_eq!((1, 4), list[0].0);
        assert_eq!(72, list[1].1.ep);
        assert_eq!((7, 2), list[1].0);
    }
}
