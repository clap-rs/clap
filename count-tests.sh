#!/bin/bash

IFS=$'\n'

touch .tmp.out

echo -n "Testing"
for TEST in $(find tests/ -type f -name "*.rs" -exec basename {} .rs \;); do
	echo -n "."
	echo -n -e "$TEST:\t" >> .tmp.out
	cargo test --test $TEST 2>&1 | grep -o -e '[0-9]* failed;' >> .tmp.out
done

echo "Done"
column -t < .tmp.out
rm .tmp.out
unset IFS
