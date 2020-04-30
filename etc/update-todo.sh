#!/bin/bash

# exit on error (-e), unset var (-u), and failed command in pipe line (-o pipefail)
set -euo pipefail

command -v rg 1>/dev/null || (echo 2>"error: ripgrep not found" && false)

mv TODO.{md,bak} || true
touch TODO.md

IFS=$'\n'
for FILE in $(rg '@TODO|FIXME' --glob='!update-todo.sh' --files-with-matches); do
    echo "- [ ] $FILE" >> TODO.md
    for LINE in $(rg -noe '@TODO([ @a-zA-Z-]+):?(.*)$' $FILE); do
        echo "    -[ ] $LINE" >> TODO.md
    done;
    for LINE in $(rg -noe 'FIXME([ @a-zA-Z-]+):?(.*)$' $FILE | rg -vi '@TODO'); do
        echo "    -[ ] $LINE" >> TODO.md
    done;
done;
unset IFS

rm TODO.bak || true
