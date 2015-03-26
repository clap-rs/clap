#!/usr/bin/env python2
import hashlib
import sys
import subprocess

failed = False

def pass_fail(good, check):
	print(good)
	if check == good:
		return "Pass"
	failed = True
	return "Fail"

def main():
	pass
	
if __name__ == '__main__':
	sys.exit(main())