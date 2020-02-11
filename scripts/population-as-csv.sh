#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

jq '[."x-log".entries[] | { cycle: .cycle, num_creatures: .num_creatures }]' $1 |
jq -r '(.[0] | keys_unsorted) as $keys | $keys, map([.[ $keys[] ]])[] | @csv' |
cat > `basename $1 .json`.csv
