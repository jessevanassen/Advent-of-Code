import { log } from '../../lib';
import { pipeline } from '../../lib/fp';
import { product, range, skip,  take } from '../../lib/fp/generators';
import { insert, iterate, Node, pop, toLinkedList, values } from './linked-list';

type Cups = number[];

const input: Cups = process.argv[2].split('').map(Number);

{
	const result = [...play(input, 100)];
	console.log('Part 1:', result.slice(1).join(''));
}

pipeline(
	play([...input, ...range(1E6 + 1, { start: input.length + 1 })], 10E6),
	skip(1),
	take(2),
	product,
	log('Part 2:'),
);

function play(cups: Cups, rounds: number): IterableIterator<number> {
	const highest = cups.length;

	const ll = toLinkedList(cups);

	const cupCache: Node<number>[] = new Array(cups.length + 1);
	for (const node of iterate(ll)) {
		cupCache[node.value] = node;
	}

	let currentCup = cupCache[cups[0]];


	for (let i = 0; i < rounds; i++) {
		const picked: Cups = [];
		for (let _ = 0; _ < 3; _++) {
			picked.push(pop(currentCup));
		}

		let destinationValue = currentCup.value;
		do {
			destinationValue = destinationValue > 1 ? destinationValue - 1 : highest;
		} while (picked.includes(destinationValue));
		const destination = cupCache[destinationValue];

		insert(destination, ...picked);

		/* Three values were just removed and re-added to a different place.
		 * That means that new nodes have been created, but the cache still
		 * points at the old nodes that aren't present in the linked list any
		 * more, so the cache needs to be updated for these new nodes. */
		for (const node of pipeline(iterate(destination), skip(1), take(3))) {
			cupCache[node.value] = node;
		}

		currentCup = currentCup.next;
	}

	return values(cupCache[1]);
}
