#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

find . -name 'log*.json' -print |
xargs cat |
jq -s -f $SCRIPTPATH/map.jq |
jq -r '(.[0] | keys_unsorted) as $keys | $keys, map([.[ $keys[] ]])[] | @csv' |
cat > _all.csv
