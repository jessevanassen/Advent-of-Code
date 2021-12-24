/**
 * @typedef {'A'|'B'|'C'|'D'} Amphipod
 * @typedef {(Amphipod|null)[]} Location
 * @typedef {{ rooms: Location[]; hallway: Location }} Layout
 * @typedef {{ layout: Layout, cost: number }} LayoutCost
 */
import { insertAll, prop, range, repeat, transpose, update, without } from 'ramda';
import { DerivedKeyMap, map, minmax, Queue, readBlocksFromStdin } from '../lib/index.js';

/** @type {{ [K in Amphipod]: number }} */
const ENERGY = Object.freeze({ A: 1, B: 10, C: 100, D: 1000 });
const EXPECTED_ROOM = Object.freeze({ A: 0, B: 1, C: 2, D: 3 });


/**
 * @returns {Layout}
 */
function parseInput() {
	const rooms = /** @type {Amphipod[][]} */ (transpose([...readBlocksFromStdin()]
			.map(line => line.replace(/[^ABCD]/g, ''))
			.filter(Boolean)
			.map(line => [...line])));
	const hallway = (repeat(null, 11));
	return { rooms, hallway };
}

/**
 * @param {Layout} layout
 * @returns {IterableIterator<{ layout: Layout, cost: number }>}
 */
function* solutions(layout) {
	const queue = Queue();
	queue.enqueue({ layout, cost: 0 });

	const seen = DerivedKeyMap(toKey);

	while (!queue.isEmpty()) {
		const head = queue.dequeue();
		if (isSolved(head.layout)) { yield head; }

		for (const possibleMove of possibleMoves(head.layout)) {
			const cost = head.cost + possibleMove.cost;

			if (seen.has(possibleMove.layout) && seen.get(possibleMove.layout) < cost) {
				continue;
			}
			seen.set(possibleMove.layout, cost);

			queue.enqueue({ layout: possibleMove.layout, cost });
		}
	}
}

/** @type {DerivedKeyMap<Layout, LayoutCost[]>} */
const moveCache = DerivedKeyMap(toKey);

/**
 * @param {Layout} layout
 * @returns {LayoutCost[]}
 */
function possibleMoves(layout) {
	if (moveCache.has(layout)) { return moveCache.get(layout); }

	const roomIndexInHallway = roomIndex => roomIndex * 2 + 2;

	const results = [];

	// Hallway to room
	for (const [i, a] of layout.hallway.entries()) {
		if (a === null) { continue; }

		// Can it reach the target room?
		const [min, max] = minmax([i, roomIndexInHallway(EXPECTED_ROOM[a])]);
		if (layout.hallway.slice(min, max + 1).filter(x => x !== null).length !== 1) {
			continue;
		}

		const hallwaySteps = max - min;

		// Does the room have a spot?
		const target = layout.rooms[EXPECTED_ROOM[a]];
		if (!(target[0] === null && target.every(x => x === null || x === a))) {
			continue;
		}

		let emptySpot = target.findIndex(x => x !== null) - 1;
		if (emptySpot === -2) { emptySpot = target.length - 1; }

		const newRoom = update(emptySpot, a, target);

		results.push({
			layout: {
				hallway: update(i, null, layout.hallway),
				rooms: update(EXPECTED_ROOM[a], newRoom, layout.rooms),
			},
			cost: (emptySpot + 1 + hallwaySteps) * ENERGY[a],
		});
	}

	// Room to hallway
	for (const [i, room] of layout.rooms.entries()) {
		if (room.every(x => x === null || x.codePointAt(0) === 0x41 + i)) { continue; }

		const indexToMove = room.findIndex(x => x !== null);
		const a = room[indexToMove];

		let hallwayStart = roomIndexInHallway(i);
		while (hallwayStart > 0 && layout.hallway[hallwayStart - 1] === null) { hallwayStart--; }

		let hallwayEnd = roomIndexInHallway(i);
		while (hallwayEnd < layout.hallway.length - 1 && layout.hallway[hallwayEnd + 1] === null) { hallwayEnd++; }

		const possibleHallwayIndices = without(
				range(0, 4).map(roomIndexInHallway),
				range(hallwayStart, hallwayEnd + 1));

		for (const hi of possibleHallwayIndices) {
			results.push({
				layout: {
					hallway: update(hi, a, layout.hallway),
					rooms: update(i, update(indexToMove, null, room), layout.rooms),
				},
				cost: ((indexToMove + 1) + Math.abs(roomIndexInHallway(i) - hi)) * ENERGY[a],
			})
		}
	}

	moveCache.set(layout, results);

	return results;
}

/**
 * @param {Layout} layout
 * @returns {string}
 */
 function toString({ hallway, rooms }) {
	const s = x => x ?? '.'
	return `${'#'.repeat(13)}
#${hallway.map(s).join('')}#
###${s(rooms[0][0])}#${s(rooms[1][0])}#${s(rooms[2][0])}#${s(rooms[3][0])}###\n` +
	transpose(rooms).slice(1).map(row => `  #${row.map(s).join('#')}#`).join('\n') + '\n' +
	`  ${'#'.repeat(9)}`;
}

/**
 * @param {Layout} layout
 * @returns {boolean}
 */
function isSolved(layout) {
	return layout.hallway.every(x => x === null) &&
			layout.rooms.every((_, i) => isRoomSolved(i, layout));
}

/**
 * @param {number} n
 * @param {Layout} layout
 * @returns {boolean}
 */
function isRoomSolved(n, layout) {
	const expectedAmphipod = ['A', 'B', 'C', 'D'][n];
	return layout.rooms[n].every(x => x === expectedAmphipod);
}

/**
 * @param {Layout} layout
 * @returns {string}
 */
function toKey({ hallway, rooms }) {
	let result = '';
	for (let i = 0; i < hallway.length; i++) {
		result += hallway[i] ?? '.';
	}
	for (let i = 0; i < rooms.length; i++) {
		for (let j = 0; j < rooms[i].length; j++) {
			result += rooms[i][j] ?? '.';
		}
	}
	return result;
}

/**
 * @param {Layout} param0
 * @returns {Layout}
 */
function unfold({ hallway, rooms }) {
	return {
		hallway,
		rooms: [
			insertAll(1, ['D', 'D'], rooms[0]),
			insertAll(1, ['C', 'B'], rooms[1]),
			insertAll(1, ['B', 'A'], rooms[2]),
			insertAll(1, ['A', 'C'], rooms[3]),
		]
	}
}

/**
 * @param {Layout} layout
 */
function solve(layout) {
	return Math.min(...map(prop('cost'), solutions(layout)));
}

const input = parseInput();

console.log('Part 1', solve(input));
console.log('Part 2', solve(unfold(input)));
