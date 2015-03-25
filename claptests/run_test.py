#!/usr/bin/env python
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
	cmd_help = [['./target/release/claptests', '-h'],
	            ['./target/release/claptests', '--help'],
	            ['./target/release/claptests', 'help']]
	proc = subprocess.Popen(cmd_help[0], stdout=subprocess.PIPE)
	proc.wait()
	out = proc.communicate()[0].decode('utf-8')
	print("Help Flag:")
	print("\tshort: {}".format(pass_fail(GOOD_HELP_HASH, out)))
	proc = subprocess.Popen(cmd_help[1], stdout=subprocess.PIPE)
	proc.wait()
	out = proc.communicate()[0].decode('utf-8')
	print("\tlong: {}".format(pass_fail(GOOD_HELP_HASH, out)))
	proc = subprocess.Popen(cmd_help[2], stdout=subprocess.PIPE)
	proc.wait()
	out = proc.communicate()[0].decode('utf-8')
	print("\tlong: {}".format(pass_fail(GOOD_HELP_HASH, out)))
	
	if failed:
		return 1


if __name__ == '__main__':
	sys.exit(main())