/**
 * @typedef {{ position: number; score: number }} Player
 * @typedef {[number, number]} NumberPair
 */

import { add, adjust } from 'ramda';
import { cycle, range, readBlocksFromStdin } from '../lib/index.js';

const initialPlayers = parseInput();

{
	let players = initialPlayers;
	const die = range({ from: 1 });

	for (const playerIndex of cycle([0, 1])) {
		const rolls = [
			die.next().value,
			die.next().value,
			die.next().value,
		];

		const steps = rolls.map(clampDie).reduce(add);

		players = adjust(playerIndex, player => move(steps, player), players);

		if (players[playerIndex].score >= 1000) {
			console.log('Part 1', rolls.at(-1) * players[otherPlayer(playerIndex)].score)
			break;
		}
	}

	/**
	 * @param {number} value
	 * @returns {number}
	 */
	function clampDie(value) {
		return ((value - 1) % 100) + 1
	}
}

{
	const rollsToUniverses = {
		3: 1,
		4: 3,
		5: 6,
		6: 7,
		7: 6,
		8: 3,
		9: 1,
	};

	/**
	 * @param {Player[]} players
	 * @param {number} currentPlayer
	 * @returns {NumberPair}
	 */
	function play(players, currentPlayer) {
		if (players[0].score >= 21) { return [1, 0]; }
		if (players[1].score >= 21) { return [0, 1]; }

		return [...range({ from: 3, to: 10 })]
				.map(roll => {
					const players_ = adjust(currentPlayer, player => move(roll, player), players);
					const next = play(players_, otherPlayer(currentPlayer));
					return multiplyNumberPair(rollsToUniverses[roll], next);
				})
				.reduce(addNumberPairs, [0, 0]);
	}

	console.log('Part 2', Math.max(...play(initialPlayers, 0)));
}

/**
 * @param {number} currentPlayer
 * @returns {number}
 */
function otherPlayer(currentPlayer) {
	return currentPlayer === 0 ? 1 : 0
}

/**
 * @param {NumberPair} x
 * @param {NumberPair} y
 * @returns {NumberPair}
 */
function addNumberPairs(x, y) {
	return [x[0] + y[0], x[1] + y[1]];
}

/**
 * @param {number} n
 * @param {NumberPair} pair
 * @returns {NumberPair}
 */
 function multiplyNumberPair(n, pair) {
	return [pair[0] * n, pair[1] * n];
}

/**
 * @param {number} steps
 * @param {Player} player
 * @returns {Player}
 */
function move(steps, { position, score }) {
	const newPosition = clampPosition(position + steps);
	return {
		position: newPosition,
		score: score + newPosition
	};
}

/**
 * @param {number} value
 * @returns {number}
 */
function clampPosition(value) {
	return ((value - 1) % 10) + 1;
}

/**
 * @returns {Player[]}
 */
function parseInput() {
	return [...readBlocksFromStdin()]
			.map(line => line.slice(line.lastIndexOf(' ') + 1))
			.map(Number)
			.map(position => ({ position, score: 0 }));
}
