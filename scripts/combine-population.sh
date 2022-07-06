#!/usr/bin/env bash

#find . -name "*.json" -not -exec jq '.cycles' {} \; -print
find . -name "log*.json" -exec  ../../scripts/population-as-csv.sh {} \;
#find . -name "log*json" -exec grep -q '"creature_max_age": 75000' {} \; -and -exec grep -q '"ring_count": 4' {} \; -and -exec grep -q '"ring_len": 7' {} \; -exec population-as-csv.sh {} \; -print

seq 0 60 |
paste -d "," -s - |
sed "s/^0/cycle/" |
sed "s/,/,f/g"> _population.csv

find log*.csv -exec wc -l {} \; |
sort -rn |
head -60 |
tr -s " " "," |
cut -d "," -f 2- |
tee _top60.txt |
xargs paste -d ";" |
grep -v cycle |
sed "s/;[0-9]*,/,/g" |
tr -s ";" "," >> _population.csv
