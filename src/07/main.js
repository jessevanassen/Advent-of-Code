import { identity, map, min, range, sum } from 'ramda';
import { readFromStdin } from '../lib/index.js';


const crabs = readFromStdin().split(',').map(n => Number.parseInt(n));

console.log('Part 1', solve(identity));
console.log('Part 2', solve(sumRange));


function solve(moveCostFn) {
	const moveCrabTo = to => from => moveCostFn(Math.abs(to - from));
	const potentialTargets = range(Math.min(...crabs), Math.max(...crabs) + 1);
	return potentialTargets
			.map(target => sum(map(moveCrabTo(target), crabs)))
			.reduce(min);
}

/**
 * Sum the numbers in the `[1, n]` range.
 *
 * @param {number} n
 * @returns {number}
 */
function sumRange(n) {
	return (n * (n + 1)) / 2;
}
