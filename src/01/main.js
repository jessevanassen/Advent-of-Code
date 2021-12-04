import { readBlocksFromStdin } from '../lib/index.js';
import { aperture, filter, length, map, pipe, sum } from 'ramda'

const input = map(Number, [...readBlocksFromStdin()]);


const countIncreases = pipe(
	aperture(2),
	filter(([x, y]) => x < y),
	length,
);

const sumSlidingWindows = pipe(
	aperture(3),
	map(sum),
);


const part1 = countIncreases;
const part2 = pipe(sumSlidingWindows, countIncreases);

console.log('Part 1:', part1(input));
console.log('Part 2:', part2(input));
