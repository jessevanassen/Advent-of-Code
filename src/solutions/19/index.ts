import { log } from "../../lib";
import { count, eq, pipe } from "../../lib/fp";
import { filter, fromEntries, join, map, split } from "../../lib/fp/generators";
import { readFromStdin } from "../../lib/fs";

const [ruleInput, input] = readFromStdin().split('\n\n');

type Predicate<T = unknown> = (x: T) => boolean;

enum Variant { Day1, Day2 }

pipe(
	split('\n'),
	filter(Validator(ruleInput, Variant.Day1)),
	count,
	log('Part 1:'),
)(input);

pipe(
	split('\n'),
	filter(Validator(ruleInput, Variant.Day2)),
	count,
	log('Part 2:'),
)(input);


function Validator(ruleInput: string, variant: Variant): Predicate<string> {
	const rules = pipe(
		split('\n'),
		map(x => x.split(': ', 2) as [string, string]),
		fromEntries,
	)(ruleInput);

	function getRule(id: string): string {
		const rule = rules[id];
		if (rule[0] === '"') {
			return rule[1];
		}

		const group = (pattern: string) => `(?:${pattern})`;

		return pipe(
			split(' | '),
			map(pipe(
				split(' '),
				map(getRule),
				join(),
				group,
			)),
			join('|'),
			group
		)(rule);
	}

	let pattern: string;

	if (variant === Variant.Day1) {
		pattern = `^${getRule('0')}$`;
	} else {
		/* Ok, this is a quite hacky way to work around the recursion. Instead of "properly"
		 * handling the recursion, just repeat the pattern for rule 11 "enough" times. I
		 * picked 25 randomly which gave me the correct output, but it also seems to work
		 * with a lot less. */
		const rule11 = (depth: number): string => depth > 0 ?
			`(?:${getRule('42')}${rule11(depth - 1)}${getRule('31')})*` :
			'';
		pattern = `^` +
			`${getRule('42')}+` +
			`(?:${rule11(25).slice(0, -1)})` +
			`$`;
	}

	return x => new RegExp(pattern).test(x);
}
