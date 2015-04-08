#!/usr/bin/env python
import sys
import subprocess

failed = False

_bin = './target/release/claptests'
cmds = {'help short:         ': ['{} -h | wc -l'.format(_bin), ['26']],
		'help long:          ': ['{} --help | wc -l'.format(_bin), ['26']],
		'help subcmd:        ': ['{} help | wc -l'.format(_bin), ['26']],
		'excluded first:     ': ['{} -f -F'.format(_bin), ['The argument -f cannot be used with one or more of the other specified arguments',
												  'USAGE:',
												  '    claptests [FLAGS] [OPTIONS] --long-option-2=option2  [POSITIONAL] [SUBCOMMANDS]',
												  'For more information try --help']],
		'excluded last:      ': ['{} -F -f'.format(_bin), ['The argument -f cannot be used with one or more of the other specified arguments',
												  'USAGE:',
												  '    claptests [FLAGS] [OPTIONS] --long-option-2=option2  [POSITIONAL] [SUBCOMMANDS]',
												  'For more information try --help']],
		'missing required:   ': ['{} -F'.format(_bin), ['One or more required arguments were not supplied',
												  'USAGE:',
												  '    claptests [FLAGS] [OPTIONS] --long-option-2=option2  [POSITIONAL] [SUBCOMMANDS]',
												  'For more information try --help']],
		'F2(ll),O(s),P:      ': ['{} --flag --flag -o some value'.format(_bin), ['flag present 2 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'F2(ss),O(s),P:      ': ['{} -f -f -o some value'.format(_bin), ['flag present 2 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'O2(ll)P:            ': ['{} --option some --option other value'.format(_bin), ['flag NOT present',
																	  'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'O2(l=l=)P:          ': ['{} --option=some --option=other value'.format(_bin), ['flag NOT present',
																	  'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'O2(ss)P:            ': ['{} -o some -o other value'.format(_bin), ['flag NOT present',
																	  'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 2 times with value: some',
																		'An option: some',
																		'An option: other',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'F2(s2),O(s),P:      ': ['{} -ff -o some value'.format(_bin), ['flag present 2 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'F(s),O(s),P:        ': ['{} -f -o some value'.format(_bin), ['flag present 1 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'F(l),O(l),P:        ': ['{} --flag --option some value'.format(_bin), ['flag present 1 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']],
		'F(l),O(l=),P:       ': ['{} --flag --option=some value'.format(_bin), ['flag present 1 times',
																	  'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'flag2 NOT present',
																		'option2 maybe present with value of: Nothing',
																		'positional2 maybe present with value of: Nothing',
																		'option3 NOT present',
																		'positional3 NOT present',
																		'option present 1 times with value: some',
																		'An option: some',
																		'positional present with value: value',
																		'subcmd NOT present']]}

def pass_fail(name, check, good):
	global failed
	print(name, end='')
	if len(good) == 1:
		if check == good:
			print('Pass')
			return
		failed = True
		print('Fail\n\tShould be: {}\n\tBut is:    {}'.format(good, check))
		return
	_failed = False
	for i, line in enumerate(check):
		if line == good[i]:
			continue
		_failed = True
		print('Fail\n\tShould be: {}\n\tBut is:    {}'.format(good[i], line))
	if _failed:
		failed = True
		return
	print('Pass')


def main():
	for cmd, cmd_v in cmds.items():
		with subprocess.Popen(cmd_v[0], shell=True, stdout=subprocess.PIPE, universal_newlines=True) as proc:
			out = proc.communicate()[0].strip()
			out = out.split('\n')
			pass_fail(cmd, out, cmd_v[1])
	if failed:
		print('One or more tests failed')
		return 1
	print('All tests passed!')

if __name__ == '__main__':
	sys.exit(main())
