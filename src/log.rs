use std::collections::HashMap;
use serde_derive::*;
use crate::creature::Creature;
use crate::program::Instr;

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
    instr_count: HashMap<Instr, u32>
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
            LogEntry {
                cycle,
                num_creatures: None,
                num_programs: None,
                programs: None,
                instr_count: HashMap::new()
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

    pub fn set_programs(&mut self, creatures: Vec<&Creature>) {
        let mut programs = HashMap::new();
        for i in 0..creatures.len() {
            let p = creatures[i].pp_program();
            let mut count = 1;
            if let Some(n) = programs.get(&p) {
                count += n;
            }
            programs.insert(p, count);
        }
        self.set(|e| e.num_programs = Some(programs.len() as u32));
        self.set(|e| e.programs = Some(programs.clone()));
    }

    pub fn incr_instr_counter(&mut self, instr: Instr) {
        self.set(|e| {
            let mut count = 1;
            if let Some(n) = e.instr_count.get(&instr) {
                count += n;
            }
            e.instr_count.insert(instr, count);
        })
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

    #[test]
    fn increments_instruction_counter() {
        let mut log = Log::new();
        log.add_entry(0);

        log.incr_instr_counter(Instr::MOV);
        log.incr_instr_counter(Instr::MOV);

        assert_eq!(2, log.entries[0].instr_count[&Instr::MOV]);
    }
}
