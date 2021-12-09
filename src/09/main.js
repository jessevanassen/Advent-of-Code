/**
 * @typedef {[number, number]} Coordinate
 */

import { add, length, multiply } from 'ramda';
import { readBlocksFromStdin } from '../lib/index.js';


const input = [...readBlocksFromStdin()].map(line => [...line].map(Number));

console.log('Part 1', [...getLowPositions()]
		.map(getValue)
		.map(add(1))
		.reduce(add));

console.log('Part 2', [...findBasins()]
		.map(length)
		.sort((a, b) => b - a)
		.slice(0, 3)
		.reduce(multiply)
);

function* findBasins() {
	for (const start of getLowPositions()) {
		const coordinates = [start];

		while (true) {
			const next = coordinates
					.flatMap(c => getAdjacentPositions(c)
							.filter(x => !coordinates.some(c_ => equals(c_, x)))
							.filter(x => getValue(x) < 9)
							.filter(x => getValue(x) > getValue(c)))
					.filter((x, i, a) => a.findIndex(x_ => equals(x, x_)) === i);
			if (next.length === 0) { break; }
			coordinates.push(...next);
		}

		yield coordinates;
	}
}

function* getLowPositions() {
	for (const position of getPositions()) {
		const value = getValue(position);
		if (getAdjacentPositions(position).every(p => getValue(p) > value)) {
			yield position;
		}
	}
}

function getValue([x, y]) {
	return input[y][x];
}

/**
 * @returns {IterableIterator<Coordinate>}
 */
function* getPositions() {
	for (let y = 0; y < input.length; y++) {
		for (let x = 0; x < input[y].length; x++) {
			yield [x, y];
		}
	}
}

/**
 * @returns {Coordinate[]}
 */
function getAdjacentPositions([x, y]) {
	/** @type {Coordinate[]} */
	const result = [];

	if (x > 0)                   result.push([x - 1, y]);
	if (x < input[0].length - 1) result.push([x + 1, y]);
	if (y > 0)                   result.push([x,     y - 1]);
	if (y < input.length - 1)    result.push([x,     y + 1]);

	return result;
}

/**
 *
 * @param {Coordinate} a
 * @param {Coordinate} b
 * @returns {boolean}
 */
function equals(a, b) {
	return a[0] === b[0] && a[1] === b[1]
}
