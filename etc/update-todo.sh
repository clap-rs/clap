#!/bin/bash

rg -h 1>/dev/null || (echo "ripgrep not found" && false)

IFS=$'\n'

mv TODO.md TODO.bak || true
touch TODO.md

for FILE in $(rg '@TODO' --ignore-file='update-todo.sh' --files-with-matches); do
    echo "- [ ] $FILE" >> TODO.md
    for LINE in $(rg -noe '@TODO([ @a-zA-Z-]+):?(.*)$' $FILE); do
        echo "    -[ ] $LINE" >> TODO.md
    done;
done;
unset IFS

rm TODO.bak || true