/**
 * @typedef {[number, number]} Coordinate
 * @typedef {number[][]} Generation
 */

import { add, clone, equals } from 'ramda';
import { findIndex, generate, map, nth, readBlocksFromStdin, scan } from '../lib/index.js';


const input = [...readBlocksFromStdin()]
		.map(line => [...line].map(Number));

console.log('Part 1',
		nth(100,
				scan(add, 0,
						map(countFlashes,
								generate(evolve, input)))));
console.log('Part 2',
		findIndex(allFlashing,
				generate(evolve, input)));


/**
 * @param {number[][]} generation
 * @returns {number[][]}
 */
function evolve(generation) {
	generation = clone(generation);

	const increment = ([x, y]) => {
		generation[y][x]++;

		if (generation[y][x] === 10) {
			for (const c of getAdjacentCoordinates([x, y])) {
				increment(c);
			}
		}
	}

	for (const c of getCoordinates()) {
		increment(c);
	}

	return generation.map(r => r.map(v => v > 9 ? 0 : v));
}

/**
 * @param {Generation} generation
 */
function countFlashes(generation) {
	return generation.flat().filter(equals(0)).length;
}

/**
 * @param {Generation} generation
 */
function allFlashing(generation) {
	return generation.flat().every(equals(0));
}

/**
 * @returns {IterableIterator<Coordinate>}
 */
function* getCoordinates() {
	for (let y = 0; y < input.length; y++) {
		for (let x = 0; x < input[y].length; x++) {
			yield [x, y];
		}
	}
}

/**
 * @param {Coordinate} c
 * @returns {Coordinate[]}
 */
function getAdjacentCoordinates(c) {
	const xMin = Math.max(0, c[0] - 1), xMax = Math.min(c[0] + 1, input[0].length - 1);
	const yMin = Math.max(0, c[1] - 1), yMax = Math.min(c[1] + 1, input.length - 1);

	/** @type {Coordinate[]} */ const result = []
	for (let y = yMin; y <= yMax; y++) {
		for (let x = xMin; x <= xMax; x++) {
			if (!equals([x, y], c)) {
				result.push([x, y]);
			}
		}
	}
	return result;
}
