import { count, pipe } from '../../lib/fp';
import { collectToArray, filter, fromEntries, map, sum } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

const SHINY_GOLD = 'shiny gold';

type BagRules = Record<string, Record<string, number>>;

const bagRules: BagRules = pipe(
	map(Rule),
	fromEntries,
)(readLinesFromStdin());

console.log('Part 1:', count(canContainShinyBag(bagRules)));
console.log('Part 2:', bagContainsCount(bagRules, SHINY_GOLD));

function Rule(line: string): [string, Record<string, number>] {
	const bag = line.match(/^\w+ \w+/)![0];
	const contains = pipe(
		(line: string) => line.matchAll(/(\d+) (\w+ \w+) bags?/g),
		map((match: RegExpMatchArray): [string, number] => [match[2], +match[1]]),
		fromEntries,
	)(line);

	return [
		bag,
		contains,
	];
}

/**
 * Returns which bags can (recursively) contain the shiny bag.
 */
function canContainShinyBag(bagRules: BagRules): string[] {
	function canContainShinyBag(color: string): boolean {
		const bagContains = Object.keys(bagRules[color]);
		return bagContains.includes(SHINY_GOLD) ? true :
			bagContains.some(canContainShinyBag);
	}
	return pipe(
		Object.keys,
		filter(canContainShinyBag),
		collectToArray,
	)(bagRules);
}

/**
 * Counts how many bags the specified bag can have.
 */
function bagContainsCount(bagRules: BagRules, color: string): number {
	return pipe(
		Object.entries,
		map(([color, count]) => count + count * bagContainsCount(bagRules, color)),
		sum,
	)(bagRules[color]);
}
