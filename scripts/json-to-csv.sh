#!/usr/bin/env bash

# convert some fields in log entries to columns in CSV file
jq '[."x-log".entries[] | {
      cycle: .cycle,
      num_creatures: .num_creatures,
      EAT: .instr_count.EAT,
      MOV: .instr_count.MOV,
      TUR: .instr_count.TUR,
      TUL: .instr_count.TUL,
      NOP: .instr_count.NOP,
      JMP: .instr_count.JMP,
      JMZ: .instr_count.JMZ,
      BFH: .instr_count.BFH,
      BFA: .instr_count.BFA
    }]' "$1" |
jq -r '(.[0] | keys_unsorted) as $keys | $keys, map([.[ $keys[] ]])[] | @csv' |
cat > `basename $1 .json`.csv
