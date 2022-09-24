#!/usr/bin/env bash

# convert some fields in log entries to columns in CSV file
jq '[."x-log".entries[] | {
      cycle: .cycle,
      num_creatures: .num_creatures,
    }]' "$1" |
jq -r '(.[0] | keys_unsorted) as $keys | $keys, map([.[ $keys[] ]])[] | @csv' |
cat > `basename $1 .json`.csv
