#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

# writes a CSV file with a summary of the runs, one row per run

find . -name 'log*.json' -print0 |
xargs -0 cat |
jq -s '[.[] | {
         id: .id,
         status: .status,
         cycles: .cycles,
         world_size: .params.world_size,
         world_end: .params.world_end,
         start_plant_count: .params.start_plant_count,
         start_population: .params.start_pop_size,
         plant_start_ep: .params.plant_start_ep,
         creature_start_ep: .params.creature_start_ep,
         creature_max_ep: .params.creature_max_ep,
         creature_max_age: .params.creature_max_age,
         eat_ep: .params.eat_ep,
         view_distance: .params.view_distance,
         prog_ring_count: .params.ring_count,
         prog_ring_size: .params.ring_size,
       }]
' |
jq -r '(.[0] | keys_unsorted) as $keys | $keys, map([.[ $keys[] ]])[] | @csv' |
cat > _all.csv
