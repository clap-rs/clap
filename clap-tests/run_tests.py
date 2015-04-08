#!/usr/bin/env python
import sys
import subprocess

failed = False

_help = '''claptests 0.0.1
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
    claptests [FLAGS] [OPTIONS] [POSITIONAL] [SUBCOMMANDS]

FLAGS:
    -f, --flag       tests flags
    -F               tests flags with exclusions
    -h, --help       Prints help information
    -v, --version    Prints version information

OPTIONS:
    -o, --option=option...          tests options
        --long-option-2=option2     tests long options with exclusions and requirements
    -O option3                      test options with specific value sets [values: fast slow]

POSITIONAL ARGUMENTS:
    positional        tests positionals
    positional2       tests positionals with exclusions and multiple
    positional3...    tests positionals with specific value sets [values: emacs vi]

SUBCOMMANDS:
    help      Prints this message
    subcmd    tests subcommands'''

_excluded = '''The argument -f cannot be used with one or more of the other specified arguments
USAGE:
    claptests [FLAGS] [OPTIONS] --long-option-2=option2  [POSITIONAL] [SUBCOMMANDS]
For more information try --help'''

_required = '''One or more required arguments were not supplied
USAGE:
    claptests [FLAGS] [OPTIONS] --long-option-2=option2  [POSITIONAL] [SUBCOMMANDS]
For more information try --help'''

_fop = '''flag present 1 times
option present 1 times with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 1 times with value: some
An option: some
positional present with value: value
subcmd NOT present'''

_f2op = '''flag present 2 times
option present 1 times with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 1 times with value: some
An option: some
positional present with value: value
subcmd NOT present'''

_o2p = '''flag NOT present
option present 2 times with value: some
An option: some
An option: other
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 2 times with value: some
An option: some
An option: other
positional present with value: value
subcmd NOT present'''

_schelp = '''claptests-subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    claptests subcmd [FLAGS] [OPTIONS] [POSITIONAL] 

FLAGS:
    -h, --help       Prints help information
    -f, --flag       tests flags
    -v, --version    Prints version information

OPTIONS:
    -o, --option=scoption...     tests options

POSITIONAL ARGUMENTS:
    scpositional    tests positionals'''

_scfop = '''flag NOT present
option NOT present
positional NOT present
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional NOT present
subcmd present
scflag present 1 times
scoption present with value: some
An scoption: some
scpositional present with value: value'''

_scf2op = '''flag NOT present
option NOT present
positional NOT present
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional NOT present
subcmd present
scflag present 2 times
scoption present with value: some
An scoption: some
scpositional present with value: value'''

_bin = './target/release/claptests'

cmds = {'help short:         ': ['{} -h'.format(_bin), _help],
		'help long:          ': ['{} --help'.format(_bin), _help],
		'help subcmd:        ': ['{} help'.format(_bin), _help],
		'excluded first:     ': ['{} -f -F'.format(_bin), _excluded],
		'excluded last:      ': ['{} -F -f'.format(_bin), _excluded],
		'missing required:   ': ['{} -F'.format(_bin), _required],
		'F2(ll),O(s),P:      ': ['{} --flag --flag -o some value'.format(_bin), _f2op],
		'F2(ss),O(s),P:      ': ['{} -f -f -o some value'.format(_bin), _f2op],
		'O2(ll)P:            ': ['{} --option some --option other value'.format(_bin), _o2p],
		'O2(l=l=)P:          ': ['{} --option=some --option=other value'.format(_bin), _o2p],
		'O2(ss)P:            ': ['{} -o some -o other value'.format(_bin), _o2p],
		'F2(s2),O(s),P:      ': ['{} -ff -o some value'.format(_bin), _f2op],
		'F(s),O(s),P:        ': ['{} -f -o some value'.format(_bin), _fop],
		'F(l),O(l),P:        ': ['{} --flag --option some value'.format(_bin), _fop],
		'F(l),O(l=),P:       ': ['{} --flag --option=some value'.format(_bin), _fop],
		'sc help short:      ': ['{} subcmd -h'.format(_bin), _schelp],
		'sc help long:       ': ['{} subcmd --help'.format(_bin), _schelp],
		'scF(l),O(l),P:      ': ['{} subcmd --flag --option some value'.format(_bin), _scfop],
		'scF(l),O(s),P:      ': ['{} subcmd --flag -o some value'.format(_bin), _scfop],
		'scF(l),O(l=),P:     ': ['{} subcmd --flag --option=some value'.format(_bin), _scfop],
		'scF(s),O(l),P:      ': ['{} subcmd -f --option some value'.format(_bin), _scfop],
		'scF(s),O(s),P:      ': ['{} subcmd -f -o some value'.format(_bin), _scfop],
		'scF(s),O(l=),P:     ': ['{} subcmd -f --option=some value'.format(_bin), _scfop],
		'scF2(s),O(l),P:     ': ['{} subcmd -ff --option some value'.format(_bin), _scf2op],
		'scF2(s),O(s),P:     ': ['{} subcmd -ff -o some value'.format(_bin), _scf2op],
		'scF2(s),O(l=),P:    ': ['{} subcmd -ff --option=some value'.format(_bin), _scf2op],
		'scF2(l2),O(l),P:    ': ['{} subcmd --flag --flag --option some value'.format(_bin), _scf2op],
		'scF2(l2),O(s),P:    ': ['{} subcmd --flag --flag -o some value'.format(_bin), _scf2op],
		'scF2(l2),O(l=),P:   ': ['{} subcmd --flag --flag --option=some value'.format(_bin), _scf2op],
		'scF2(s2),O(l),P:    ': ['{} subcmd -f -f --option some value'.format(_bin), _scf2op],
		'scF2(s2),O(s),P:    ': ['{} subcmd -f -f -o some value'.format(_bin), _scf2op],
		'scF2(s2),O(l=),P:   ': ['{} subcmd -f -f --option=some value'.format(_bin), _scf2op]
		}

def pass_fail(name, check, good):
	global failed
	print(name, end='')
	if check == good:
		print('Pass')
		return
	failed = True
	print('Fail\n\tShould be: {}\n\tBut is:    {}'.format(good, check))


def main():
	for cmd, cmd_v in cmds.items():
		with subprocess.Popen(cmd_v[0], shell=True, stdout=subprocess.PIPE, universal_newlines=True) as proc:
			out = proc.communicate()[0].strip()
			pass_fail(cmd, out, cmd_v[1])
	if failed:
		print('One or more tests failed')
		return 1
	print('All tests passed!')

if __name__ == '__main__':
	sys.exit(main())
