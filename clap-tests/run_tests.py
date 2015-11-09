#!/usr/bin/env python
import sys
import subprocess
import re
import difflib

failed = False

_ansi = re.compile(r'\x1b[^m]*m')

_help = '''claptests v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
\tclaptests [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -f, --flag       tests flags
    -F               tests flags with exclusions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests mutliple values, not mult occs
        --multvalsmo <one> <two>     Tests mutliple values, not mult occs
    -o, --option <opt>...            tests options
        --long-option-2 <option2>    tests long options with exclusions
    -O, --Option <option3>           tests options with specific value sets [values: fast slow]

ARGS:
    positional        tests positionals
    positional2       tests positionals with exclusions
    positional3...    tests positionals with specific values

SUBCOMMANDS:
    help      Prints this message
    subcmd    tests subcommands'''

_version = "claptests v1.4.8"

_sc_dym_usage = '''error: The subcommand 'subcm' isn't valid
\tDid you mean 'subcmd' ?

If you received this message in error, try re-running with 'claptests -- subcm'

USAGE:
\tclaptests [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help'''

_arg_dym_usage = '''error: The argument '--optio' isn't valid
\tDid you mean --option ?

USAGE:
\tclaptests --option <opt>...

For more information try --help'''

_pv_dym_usage = '''error: 'slo' isn't a valid value for '--Option <option3>'
\t[valid values: fast slow]

    Did you mean 'slow' ?

USAGE:
\tclaptests --Option <option3>

For more information try --help'''

_excluded = '''error: The argument '--flag' cannot be used with '-F'

USAGE:
\tclaptests [positional2] -F --long-option-2 <option2>

For more information try --help'''

_excluded_l = '''error: The argument '-f' cannot be used with '-F'

USAGE:
\tclaptests [positional2] -F --long-option-2 <option2>

For more information try --help'''

_required = '''error: The following required arguments were not supplied:
\t[positional2]
\t--long-option-2 <option2>

USAGE:
\tclaptests [positional2] -F --long-option-2 <option2>

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
\tclaptests subcmd [FLAGS] [OPTIONS] [--] [ARGS]

FLAGS:
    -f, --flag       tests flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --option <scoption>...    tests options

ARGS:
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
flag present 1 times
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
flag present 2 times
scoption present with value: some
An scoption: some
scpositional present with value: value'''

_min_vals_few = '''error: The argument '--minvals2 <minvals>...' requires at least 2 values, but 1 was provided

USAGE:
\tclaptests --minvals2 <minvals>...

For more information try --help'''

_exact = '''flag NOT present
option NOT present
positional NOT present
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional NOT present
subcmd NOT present'''

_max_vals_more = '''flag NOT present
option NOT present
positional present with value: too
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional present with value: too
subcmd NOT present'''

_mult_vals_more = '''error: The argument '--multvals <one> <two>' was supplied more than once, but does not support multiple occurrences

USAGE:
\tclaptests --multvals <one> <two>

For more information try --help'''

_mult_vals_few = '''error: The argument '--multvals <one> <two>' requires 2 values, but 1 was provided

USAGE:
\tclaptests --multvals <one> <two>

For more information try --help'''

_mult_vals_2m1 = '''error: The argument '--multvalsmo <one> <two>' requires 2 values, but 1 was provided

USAGE:
\tclaptests --multvalsmo <one> <two>

For more information try --help'''

_bin = './target/release/claptests'

cmds = {'help short:         ': ['{} -h'.format(_bin), _help, 0],
		'help long:          ': ['{} --help'.format(_bin), _help, 0],
		'version short:      ': ['{} -V'.format(_bin), _version, 0],
		'version long:       ': ['{} --version'.format(_bin), _version, 0],
		'help subcmd:        ': ['{} help'.format(_bin), _help, 0],
		'missing required:   ': ['{} -F'.format(_bin), _required, 1],
		'max_vals too many:  ': ['{} --maxvals3 some other value too'.format(_bin), _max_vals_more, 0],
		'max_vals exact:     ': ['{} --maxvals3 some other value'.format(_bin), _exact, 0],
		'max_vals less:      ': ['{} --maxvals3 some other'.format(_bin), _exact, 0],
		'min_vals more:      ': ['{} --minvals2 some other value too'.format(_bin), _exact, 0],
		'min_vals exact:     ': ['{} --minvals2 some value'.format(_bin), _exact, 0],
		'min_vals too few:   ': ['{} --minvals2 some'.format(_bin), _min_vals_few, 1],
		'mult_vals too many: ': ['{} --multvals some other --multvals some other'.format(_bin), _mult_vals_more, 1],
		'mult_vals too few:  ': ['{} --multvals some'.format(_bin), _mult_vals_few, 1],
		'mult_vals exact:    ': ['{} --multvals some other'.format(_bin), _exact, 0],
		'mult_valsmo x2:     ': ['{} --multvalsmo some other --multvalsmo some other'.format(_bin), _exact, 0],
		'mult_valsmo x2-1:   ': ['{} --multvalsmo some other --multvalsmo some'.format(_bin), _mult_vals_2m1, 1],
		'mult_valsmo x1:     ': ['{} --multvalsmo some other'.format(_bin), _exact, 0],
		'F2(ss),O(s),P:      ': ['{} value -f -f -o some'.format(_bin), _f2op, 0],
		'arg dym:            ': ['{} --optio=foo'.format(_bin), _arg_dym_usage, 1],
		'O2(ll)P:            ': ['{} value --option some --option other'.format(_bin), _o2p, 0],
		'O2(l=l=)P:          ': ['{} value --option=some --option=other'.format(_bin), _o2p, 0],
		'O2(ss)P:            ': ['{} value -o some -o other'.format(_bin), _o2p, 0],
		'F2(s2),O(s),P:      ': ['{} value -ff -o some'.format(_bin), _f2op, 0],
		'F(s),O(s),P:        ': ['{} value -f -o some'.format(_bin), _fop, 0],
		'F(l),O(l),P:        ': ['{} value --flag --option some'.format(_bin), _fop, 0],
		'F(l),O(l=),P:       ': ['{} value --flag --option=some'.format(_bin), _fop, 0],
		'sc dym:             ': ['{} subcm'.format(_bin), _sc_dym_usage, 1],
		'sc help short:      ': ['{} subcmd -h'.format(_bin), _schelp, 0],
		'sc help long:       ': ['{} subcmd --help'.format(_bin), _schelp, 0],
		'scF(l),O(l),P:      ': ['{} subcmd value --flag --option some'.format(_bin), _scfop, 0],
		'scF(l),O(s),P:      ': ['{} subcmd value --flag -o some'.format(_bin), _scfop, 0],
		'scF(l),O(l=),P:     ': ['{} subcmd value --flag --option=some'.format(_bin), _scfop, 0],
		'scF(s),O(l),P:      ': ['{} subcmd value -f --option some'.format(_bin), _scfop, 0],
		'scF(s),O(s),P:      ': ['{} subcmd value -f -o some'.format(_bin), _scfop, 0],
		'scF(s),O(l=),P:     ': ['{} subcmd value -f --option=some'.format(_bin), _scfop, 0],
		'scF2(s),O(l),P:     ': ['{} subcmd value -ff --option some'.format(_bin), _scf2op, 0],
		'scF2(s),O(s),P:     ': ['{} subcmd value -ff -o some'.format(_bin), _scf2op, 0],
		'scF2(s),O(l=),P:    ': ['{} subcmd value -ff --option=some'.format(_bin), _scf2op, 0],
		'scF2(l2),O(l),P:    ': ['{} subcmd value --flag --flag --option some'.format(_bin), _scf2op, 0],
		'scF2(l2),O(s),P:    ': ['{} subcmd value --flag --flag -o some'.format(_bin), _scf2op, 0],
		'scF2(l2),O(l=),P:   ': ['{} subcmd value --flag --flag --option=some'.format(_bin), _scf2op, 0],
		'scF2(s2),O(l),P:    ': ['{} subcmd value -f -f --option some'.format(_bin), _scf2op, 0],
		'scF2(s2),O(s),P:    ': ['{} subcmd value -f -f -o some'.format(_bin), _scf2op, 0],
		'scF2(s2),O(l=),P:   ': ['{} subcmd value -f -f --option=some'.format(_bin), _scf2op, 0]
		}

def pass_fail(name, cmd, check, good):
	sys.stdout.write(name)
	if check == good:
		print('Pass')
		return 0
	print('Fail')
	print('\n\n{}\n# Should be:\n$ {}\n{}\n\n{}\n# But is:\n$ {}\n{}\n\n'.format('#'*25, cmd, good, '#'*25, cmd, check))
	for line in difflib.context_diff(good.splitlines(), check.splitlines(), fromfile="Should Be", tofile="Currently Is", lineterm=""):
		print(line)
	print()
	return 1


def main():
	num_failed = 0
	total = len(cmds)
	for cmd, cmd_v in cmds.items():
		proc = subprocess.Popen(cmd_v[0], shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
		out, err = proc.communicate()
		out = _ansi.sub('', out.strip())
		err = _ansi.sub('', err.strip())
		rc = proc.returncode
		if rc != cmd_v[-1]:
			print('{}Fail (Exit Code={}; Should be={})'.format(cmd, rc, cmd_v[-1]))
			num_failed += 1
			continue
		if out and rc == cmd_v[-1]:
			num_failed += pass_fail(cmd, cmd_v[0], out, cmd_v[1])
		elif rc == cmd_v[-1]:
			num_failed += pass_fail(cmd, cmd_v[0], err, cmd_v[1])

	print()
	if num_failed:
		print('{}/{} tests failed'.format(num_failed, total))
		return 1
	print('{}/{} tests passed!'.format(total, total))

if __name__ == '__main__':
	sys.exit(main())
