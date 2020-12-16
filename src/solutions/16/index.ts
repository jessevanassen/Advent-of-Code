import { isBetween } from "../../lib";
import { mapObject, not, pipe } from "../../lib/fp";
import { all, collectToArray, contains, entries, filter, flatten, fromEntries, map, reduce, skip, split, sum, transpose, zipWithIndex } from "../../lib/fp/generators";
import { readFromStdin } from "../../lib/fs";

type Range = [number, number];
type Predicate<T> = (x: T) => boolean;

interface Input {
	fields: Record<string, Range[]>;
	yourTicket: number[];
	nearbyTickets: number[][];
}

const input = parseInput();

console.log('Part 1:', solvePart1(input));
console.log('Part 2:', solvePart2(input));

function solvePart1(input: Input) {
	return pipe(
		filter(not(all(validForAnyField(input)))),
		flatten,
		filter(not(validForAnyField(input))),
		sum,
	)(input.nearbyTickets);
}

function solvePart2(input: Input) {
	const fieldPredicates = mapObject(fieldPredicate)(input.fields);
	const fieldNames = Object.keys(input.fields);

	const notInvalidTickets: number[][] = input.nearbyTickets
		.filter(all(validForAnyField(input)));

	const columns = [...transpose(notInvalidTickets)];
	const designation = new Map<string, number>();

	while (designation.size < fieldNames.length) {
		for (const [column, index] of zipWithIndex(columns)) {
			if (contains(index)(designation.values())) {
				continue;
			}

			const possibleFields = pipe(
				() => fieldPredicates,
				entries,
				filter(([name]) => !designation.has(name)),
				filter(([, predicate]) => column.every(predicate)),
				map(([name]) => name),
				collectToArray,
			)(null);

			if (possibleFields.length === 1) {
				designation.set(possibleFields[0], index);
			}
		}
	}

	return [...designation.entries()]
		.filter(([name]) => name.startsWith('departure'))
		.map(([, column]) => input.yourTicket[column])
		.reduce((a, b) => a * b, 1);
}

function validForAnyField(input: Input): Predicate<number> {
	return pipe(
		() => Object.values(input.fields),
		map(fieldPredicate),
		reduce(or),
	)(null);
}

function fieldPredicate(field: Range[]): Predicate<number> {
	return pipe(
		() => field,
		map(([min, max]) => isBetween(min, max)),
		reduce(or),
	)(null);
}

function or<T>(first: Predicate<T>, second: Predicate<T>): Predicate<T> {
	return x => first(x) || second(x);
}

function parseInput(): Input {
	const blocks = readFromStdin().split('\n\n');

	const parseField = (line: string): [string, Range[]] => {
		const [key, ranges] = line.split(': ');

		const parsedRanges = pipe(
			() => ranges.matchAll(/(\d+)-(\d+)/g),
			map((match: RegExpMatchArray): Range => [Number.parseInt(match[1]), Number.parseInt(match[2])]),
			collectToArray,
		)(null)

		return [key, parsedRanges];
	};

	const fields = pipe(
		split('\n'),
		map(parseField),
		fromEntries,
	)(blocks[0]);

	const parseNumbers = pipe(
		split('\n'),
		skip(1),
		map(pipe(
			split(','),
			map(Number),
			collectToArray,
		)),
		collectToArray,
	);

	const yourTicket = parseNumbers(blocks[1])[0];
	const nearbyTickets = parseNumbers(blocks[2]);

	return { fields, yourTicket, nearbyTickets };
}
