/**
 * @typedef {string} Cave
 * @typedef {(cave: Cave, visited: Cave[]) => boolean} VisitAllowed
 */

import { count, readBlocksFromStdin } from '../lib/index.js';

const graph = parseInput();

console.log('Part 1', count(findPaths(smallCaveRevisitNotAllowed)));
console.log('Part 2', count(findPaths(smallCaveRevisitAllowedOnce)));

/**
 * @param {VisitAllowed} visitAllowed
 * @returns {IterableIterator<Cave[]>}
 */
function findPaths(visitAllowed) {
	function* findPaths_(acc) {
		for (const connection of graph.get(acc.at(-1))) {
			if (connection === 'end') {
				yield [...acc, connection];
				continue;
			}

			if (visitAllowed(connection, acc)) {
				yield* findPaths_([...acc, connection]);
			}
		}
	}
	return findPaths_(['start']);
}

/**
 * @param {Cave} cave
 * @param {Cave[]} visited
 * @returns {boolean}
 */
function smallCaveRevisitNotAllowed(cave, visited) {
	return !isSmall(cave) || !visited.includes(cave);
}

/**
 * @param {Cave} cave
 * @param {Cave[]} visited
 * @returns {boolean}
 */
function smallCaveRevisitAllowedOnce(cave, visited) {
	if (smallCaveRevisitNotAllowed(cave, visited)) { return true; }

	const smallCavesRevisited = [...visited.filter(isSmall), cave];
	return smallCavesRevisited.length - new Set(smallCavesRevisited).size <= 1;
}

/**
 * @param {Cave} cave
 */
function isSmall(cave) {
	return cave.toLowerCase() === cave;
}

/**
 * @returns {Map<Cave, Set<Cave>>}
 */
function parseInput() {
	const graph = new Map();

	for (const line of readBlocksFromStdin()) {
		const c = line.split('-', 2);

		for (const [a, b] of [c, [...c].reverse()]) {
			if (!graph.has(a)) { graph.set(a, new Set()); }
			if (a === 'end' || b == 'start') { continue; }
			graph.get(a).add(b);
		}
	}

	return graph;
}
