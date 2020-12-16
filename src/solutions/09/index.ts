import { combinations2 } from '../../lib';
import { eq, pipe } from '../../lib/fp';
import { any, collectToArray, map, range, sum } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

const preambleSize = +process.argv[2];

const input = pipe(
	map(Number),
	collectToArray,
)(readLinesFromStdin());

const invalidNumber = findInvalidNumber(preambleSize, input);
console.log('Part 1:', invalidNumber);

const r = findRangeWhichSumsTo(invalidNumber, input);
console.log('Part 2:', Math.min(...r) + Math.max(...r));

function findInvalidNumber(preambleSize: number, input: number[]): number {
	for (const i of range(input.length, { start: preambleSize })) {
		const sumsToNumber = pipe(
			() => combinations2(input.slice(i - preambleSize, i)),
			map(sum),
			any(eq(input[i])),
		)(null);
		if (!sumsToNumber) {
			return input[i];
		}
	}
	return -1;
}

function findRangeWhichSumsTo(n: number, input: number[]) {
	for (const i of range(input.length - 1)) {
		for (const j of range(input.length, { start: i + 1 })) {
			const s = sum(input.slice(i, j + 1));

			if (s === n) {
				return input.slice(i, j + 1);
			}

			if (s > n) {
				break;
			}
		}
	}

	return [];
}
