/**
 * @typedef { {
 *   width: number;
 *   height: number;
 *   getValue: (v: Vector2) => number
 * } } Grid
 */
import { minBy } from 'ramda';
import { DerivedKeyMap, DerivedKeySet, readBlocksFromStdin, Vector2 } from '../lib/index.js';

const input = parseInput();

console.log('Part 1:', findShortestPath(input));
console.log('Part 2:', findShortestPath(LoopingGrid(5, input)));

/**
 * Using Dijkstra's algorithm
 * @param {Grid} grid
 */
function findShortestPath({ width, height, getValue }) {
	const from = Vector2({ x: 0, y: 0 });
	const to = Vector2({ x: width - 1, y: height - 1 });

	/** @type {(vector: Vector2) => number} */
	const keyFn = ({ x, y }) => (x << 16) | y;

	/** @type {DerivedKeyMap<Vector2, number>} */ const distance = DerivedKeyMap(keyFn);
	/** @type {DerivedKeySet<Vector2>} */ const queue = DerivedKeySet(keyFn);
	/** @type {DerivedKeySet<Vector2>} */ const seen = DerivedKeySet(keyFn);

	/** @type {(v: Vector2) => number} */
	const getDistance = v => distance.get(v) ?? Number.POSITIVE_INFINITY;

	queue.add(from);
	distance.set(from, 0);

	while (queue.size > 0) {
		const u = [...queue.values()].reduce(minBy(getDistance));
		queue.delete(u);
		seen.add(u);

		for (const neighbor of neighbors(width, height, u)) {
			if (seen.has(neighbor)) { continue; }
			queue.add(neighbor);

			const alt = getDistance(u) + getValue(neighbor);

			if (alt < getDistance(neighbor)) {
				distance.set(neighbor, alt);
			}
		}
	}

	return distance.get(to);
}

/**
 * @param {number} width
 * @param {number} height
 * @param {Vector2} vector2
 * @returns {Vector2[]}
 */
function neighbors(width, height, { x, y }) {
	const result = [];

	if (x > 0)          result.push({ x: x - 1, y });
	if (y > 0)          result.push({ x,        y: y - 1 });
	if (x < width - 1)  result.push({ x: x + 1, y });
	if (y < height - 1) result.push({ x,        y: y + 1 });

	return result;
}

function parseInput() {
	const grid = [...readBlocksFromStdin()]
			.map(line => [...line]
					.map(c => Number(c)));
	return {
		width: grid[0].length,
		height: grid.length,
		getValue: ({ x, y }) => grid[y][x],
	};
}

/**
 * @param {number} size
 * @param {Grid} original
 * @returns {Grid}
 */
function LoopingGrid(size, { width, height, getValue }) {
	return {
		width: width * size,
		height: height * size,
		getValue: ({ x, y }) => {
			const originalValue = getValue({ x: x % width, y: y % width });
			const additionalCost = ~~(x / width) + ~~(y / height);
			const v = originalValue + additionalCost;
			return (v % 10) + ~~(v / 10);
		}
	}
}
