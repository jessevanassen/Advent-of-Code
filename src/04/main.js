/**
 * @typedef {number[][]} Card
 */

import { all, any, sum, transpose } from 'ramda';
import { range, readBlocksFromStdin } from '../lib/index.js';

const { numbers: numbersDrawn, cards } = readInput();


console.log('Part 1:', findWinner(cards).score);

{
	const noWinners = new Set(cards);
	while (noWinners.size > 1) {
		noWinners.delete(findWinner([...noWinners]).card);
	}
	console.log('Part 2:', findWinner([...noWinners]).score);
}

/**
 * @param {Card[]} cards
 * @returns {{ score: number; card: Card }}
 */
function findWinner(cards) {
	const drawn = new Set();
	for (let i = 0; i < numbersDrawn.length; i++) {
		drawn.add(numbersDrawn[i])

		for (const card of cards) {
			if (hasBingo(drawn, card)) {
				const without = card.flat().filter(x => !drawn.has(x));
				return {
					score: sum(without) * numbersDrawn[i],
					card
				}
			}
		}
	}
}

/**
 * @param {Set<number>} drawn
 * @param {Card} card
 * @returns {boolean}
 */
function hasBingo(drawn, card) {
	const allDrawn = xs => all(x => drawn.has(x), xs);
	return any(allDrawn, card) || any(allDrawn, transpose(card));
}

/**
 * @returns {{ numbers: number[]; cards: Card[] }}
 */
function readInput() {
	const input = [...readBlocksFromStdin()];
	const numbers = input[0].split(',').map(Number);

	const cards = [...range({ from: 2, to: input.length, step: 6 })]
			.map(i => input.slice(i, i + 5)
					.map(row => row.trim().split(/\s+/).map(Number)));

	return { numbers, cards };
}
