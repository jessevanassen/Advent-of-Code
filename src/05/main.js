import { filter, groupBy, length, map, pipe, range, repeat, unnest, values, zip } from 'ramda';
import { readBlocksFromStdin, Vector2 } from '../lib/index.js';


/**
 * @typedef {[Vector2, Vector2]} Line
 */

/** @type {Line[]} */
const lines = [...readBlocksFromStdin()]
	.map(row => row.split(' -> ', 2).map(Vector2.parse));


console.log('Part 1', countOverlappingPoints(filter(isStraight, lines)));
console.log('Part 2', countOverlappingPoints(lines));


function isStraight([{ x: x1, y: y1 }, { x: x2, y: y2 }]) {
	return x1 === x2 || y1 === y2;
}

/**
 * @param {Line[]} lines
 */
function countOverlappingPoints(lines) {
	return pipe(
		map(pointsInLine),
		unnest,
		groupBy(Vector2.toString),
		values,
		map(length),
		filter(l => l > 1),
		length,
	)(lines);
}

/**
 * @param {Line} param0
 */
function pointsInLine([{ x: x1, y: y1 }, { x: x2, y: y2 }]) {
	const xRange = x1 === x2 ? repeat(x1, Math.abs(y2 - y1) + 1) : rangeInclusive(x1, x2);
	const yRange = y1 === y2 ? repeat(y1, Math.abs(x2 - x1) + 1) : rangeInclusive(y1, y2);
	return map(
			([x, y]) => Vector2({ x, y }),
			zip(xRange, yRange),
	);
}

/**
 * Creates an inclusive range of numbers.
 *
 * Also supports reverse ranges (e.g. [3, 2, 1])
 *
 * @param {number} x
 * @param {number} y
 * @returns {number[]}
 */
function rangeInclusive(x, y) {
	return (x > y) ?
			rangeInclusive(y, x).reverse() :
			range(x, y + 1);
}
