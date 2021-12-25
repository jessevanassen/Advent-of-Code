/**
 * @typedef {'>' | 'v' | '.'} Cell
 * @typedef {Cell[][]} Grid
 **/

import { repeat, times } from 'ramda';
import { count, readBlocksFromStdin } from '../lib/index.js';

/**
 * @returns {{ width: number; height: number; grid: Grid }}
 */
function parseInput() {
	const grid = /** @type {Grid} */ ([...readBlocksFromStdin()]
			.map(row => [...row]));
	const height = grid.length, width = grid[0].length;
	return { width, height, grid };
}

/**
 * @param {Grid} grid
 * @returns {{ value: Grid, done: boolean }}
 */
function evolve(grid) {
	const next = emptyGrid(width, height);
	let changed = false;

	for (const [x, y] of positions(width, height)) {
		if (grid[y][x] !== '>') { continue; }

		const nextX = (x + 1) % width;
		if (grid[y][nextX] === '.') {
			next[y][nextX] = '>';
			changed = true;
		} else {
			next[y][x] = '>';
		}
	}

	for (const [x, y] of positions(width, height)) {
		if (grid[y][x] !== 'v') { continue; }

		const nextY = (y + 1) % height;
		if (grid[nextY][x] !== 'v' && next[nextY][x] === '.') {
			next[nextY][x] = 'v';
			changed = true;
		} else {
			next[y][x] = 'v';
		}
	}

	return { value: next, done: !changed };
}

/**
 * @param {number} width
 * @param {number} height
 * @returns {IterableIterator<[number, number]>}
 */
function* positions(width, height) {
	for (let y = 0; y < height; y++) {
		for (let x = 0; x < width; x++) {
			yield [x, y];
		}
	}
}

/**
 * @param {number} width
 * @param {number} height
 * @returns {Grid}
 */
function emptyGrid(width, height) {
	return times(() => repeat('.', width), height);
}

/**
 * @param {Grid} grid
 * @returns {IterableIterator<Grid>}
 */
function* evolutions(grid) {
	while (true) {
		yield grid;
		const next = evolve(grid);
		if (next.done) { return; }
		grid = next.value;
	}
}


const { width, height, grid } = parseInput();

console.log('Part 1:', count(evolutions(grid)));
