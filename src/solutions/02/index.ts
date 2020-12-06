import { count, eq, inspect, pipe, truthy } from "../../lib/fp";
import { collect, filter, map, split } from "../../lib/fp/generators";
import { readLinesFromStdin } from "../../lib/fs";

const pattern = /^(\d+)-(\d+) ([a-z]): ([a-z]+)$/;

const passwords = pipe(
	map(match(pattern)),
	filter(truthy),
	map(PasswordPolicy),
	collect,
)(readLinesFromStdin());


pipe(
	filter(validatePolicy1),
	count,
	log('Part 1: '),
)(passwords);

pipe(
	filter(validatePolicy2),
	count,
	log('Part 2: '),
)(passwords);


function validatePolicy1({ pos1, pos2, character, password }: PasswordPolicy) {
	return pipe(
		split(),
		filter(eq(character)),
		count,
		isBetween(pos1, pos2),
	)(password);
}

function validatePolicy2({ pos1, pos2, character, password }: PasswordPolicy) {
	return (password[pos1 - 1] === character) !== (password[pos2 - 1] === character);
}

type PasswordPolicy = ReturnType<typeof PasswordPolicy>;
function PasswordPolicy([pos1, pos2, character, password]: string[]) {
	return {
		pos1: Number.parseInt(pos1),
		pos2: Number.parseInt(pos2),
		character, password
	};
}

function match(pattern: RegExp) {
	return (input: string) => input.match(pattern)?.slice(1);
}

function isBetween(start: number, endInclusive: number) {
	return (n: number) => start <= n && n <= endInclusive;
}

function log(prefix: string = '') {
	return (x: unknown) => console.log(`${prefix}${x}`);
}
