import { repeat, sum } from 'ramda';
import { readFromStdin } from '../lib/index.js';


const input = readFromStdin().split(',').map(n => Number.parseInt(n));

const generations = [repeat(0, 9)];
for (const x of input) {
	generations[0][x]++;
}

for (let i = 0; i < 256; i++) {
	generations.push(next(generations[i]))
}

console.log('Part 1', sum(generations[80]));
console.log('Part 2', sum(generations[256]));

/**
 * @param {number[]} generation
 * @returns {number[]}
 */
function next(generation) {
	return [
		...generation.slice(1, 7),
		generation[7] + generation[0],
		generation[8],
		generation[0],
	];
}
