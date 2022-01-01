#!/usr/bin/env python3

import argparse
import re
import pathlib
import sys


_STDIO = pathlib.Path("-")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", type=pathlib.Path, default="CHANGELOG.md")
    parser.add_argument("--tag", required=True)
    parser.add_argument("-o", "--output", type=pathlib.Path, required=True)
    args = parser.parse_args()

    if args.input == _STDIO:
        lines = sys.stdin.readlines()
    else:
        with args.input.open() as fh:
            lines = fh.readlines()
    version = args.tag.lstrip("v")

    note_lines = []
    for line in lines:
        if line.startswith("## ") and version in line:
            note_lines.append(line)
        elif note_lines and line.startswith("## "):
            break
        elif note_lines:
            note_lines.append(line)

    notes = "".join(note_lines).strip()
    if args.output == _STDIO:
        print(notes)
    else:
        args.output.write_text(notes)


if __name__ == "__main__":
    main()
