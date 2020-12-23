import { log } from '../../lib';
import { pipe, pipeline } from '../../lib/fp';
import { collectToArray, map, skip, split, sum, zipWithIndex } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

type Deck = number[];
type Decks = [Deck, Deck];
type Player = 0 | 1;

const decks = parseDecks();

pipeline(
	playGame(decks),
	winner,
	score,
	log('Part 1:'),
);

pipeline(
	playRecursiveGame(decks),
	winner,
	score,
	log('Part 2:'),
);

function playRecursiveGame(decks: Decks): Decks {
	const configurations: Decks[] = [];

	do {
		if (configurations.some(deckEqualTo(decks))) {
			return [decks[0], []];
		}
		configurations.push(decks);

		if (decks[0].length > decks[0][0] && decks[1].length > decks[1][0]) {
			const winner = playRecursiveGame([
				decks[0].slice(1, decks[0][0] + 1),
				decks[1].slice(1, decks[1][0] + 1)],
			)[0].length > 0 ? 0 : 1;
			decks = transferTo(winner, decks);
		} else {
			decks = playRound(decks);
		}
	} while (decks[0].length > 0 && decks[1].length > 0);

	return decks;
}

function deckEqualTo(x: Decks) {
	return function(y: Decks): boolean {
		if (x[0].length !== y[0].length || x[1].length !== y[1].length) {
			return false;
		}

		return x[0].every((_, i) => x[0][i] === y[0][i]) &&
			x[1].every((_, i) => x[1][i] === y[1][i]);
	};
}

function playGame(decks: Decks): Decks {
	do {
		decks = playRound(decks);
	} while (decks[0].length > 0 && decks[1].length > 0);
	return decks;
}

function playRound(decks: Decks): Decks {
	return transferTo(decks[0][0] > decks[1][0] ? 0 : 1, decks);
}

function transferTo(player: Player, [deck0, deck1]: Decks): Decks {
	return player === 0 ?
		[
			[...deck0.slice(1), deck0[0], deck1[0]],
			deck1.slice(1),
		] :
		transferTo(0, [deck1, deck0]).reverse() as Decks;
}

function winner([deck0, deck1]: Decks): Deck {
	return deck0.length > 0 ? deck0 : deck1;
}

function score(deck: Deck): number {
	return pipeline(
		deck.reverse(),
		zipWithIndex,
		map(([value, index]) => value * (index + 1)),
		sum,
	);
}

function parseDecks(): Decks {
	return pipeline(
		readBlocksFromStdin('\n\n'),
		map(pipe(
			split('\n'),
			skip(1),
			map(Number),
			collectToArray,
		)),
		collectToArray,
	) as Decks;
}
