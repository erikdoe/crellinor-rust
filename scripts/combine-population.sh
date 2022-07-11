#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

# convert population from JSON files to CSV
find . -name "log*.json" -exec  $SCRIPTPATH/population-as-csv.sh {} \;

# write header to CSV file
seq 0 40 |
paste -d "," -s - |
sed "s/^0/cycle/" |
sed "s/,/,f/g"> _population.csv

# write population of top 40 runs as columns to CSV file and create text file
# that lists the filenames of the top 40 runs in same order as the columns
find log*.csv -exec wc -l {} \; |
sort -rn |
head -40 |
tr -s " " "," |
cut -d "," -f 2- |
tee _top40.txt |
xargs paste -d ";" |
grep -v cycle |
sed "s/;[0-9]*,/,/g" |
tr -s ";" "," >> _population.csv
