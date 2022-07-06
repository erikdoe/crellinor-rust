use std::collections::HashMap;
use serde_derive::*;
use crate::creature::Creature;

#[derive(Serialize)]
pub struct Log {
    pub entries: Vec<LogEntry>,
    pub total_cycles: u64,
}


#[derive(Serialize)]
pub struct LogEntry {
    cycle: u64,
    num_creatures: Option<u32>,
    num_programs: Option<u32>,
    programs: Option<HashMap<String, u32>>,
}



impl Log {

    pub fn new() -> Log {
        Log {
            entries: Vec::new(),
            total_cycles: 0,
        }
    }

    pub fn add_entry(&mut self, cycle: u64) {
        self.entries.push(
            LogEntry{
                cycle,
                num_creatures: None,
                num_programs: None,
                programs: None
            });
    }

    fn set<F>(&mut self, changefn: F) where F: Fn(&mut LogEntry) {
        if let Some(entry) = self.entries.last_mut() {
            changefn(entry);
        }
    }

    pub fn set_num_creatures(&mut self, n: u32) {
        self.set(|e| e.num_creatures = Some(n));
    }

    pub fn set_programs(&mut self, creatures: Vec<&Creature>, ring_size: usize) {
        let mut programs = HashMap::new();
        for i in 0..creatures.len() {
            let p = self.program_as_string(creatures[i], ring_size);
            let mut count = 1;
            if let Some(n) = programs.get(&p) {
                count += n;
            }
            programs.insert(p, count);
        }
        self.set(|e| e.num_programs = Some(programs.len() as u32));
        self.set(|e| e.programs = Some(programs.clone()));
    }


    pub fn program_as_string(&self, creature: &Creature, ring_size: usize) -> String {
        let mut out = String::new();
        for i in 0..creature.program.len() {
            out.push_str(&format!("{:?} ", creature.program[i]));
            if (i + 1) % ring_size == 0 {
                out.push_str("  ");
            }
        }
        format!("{:5}  {}", creature.age(), out)
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_values_to_last_entry() {
        let mut log = Log::new();

        log.add_entry(0);
        log.set_num_creatures(12);
        log.add_entry(1);
        log.set_num_creatures(20);

        assert_eq!(Some(12), log.entries[0].num_creatures);
        assert_eq!(Some(20), log.entries[1].num_creatures);

    }
}
