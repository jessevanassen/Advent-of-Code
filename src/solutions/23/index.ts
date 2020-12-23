import { log } from '../../lib';
import { pipeline } from '../../lib/fp';
import { forEach, last, range, reduce, take } from '../../lib/fp/generators';

type Cups = number[];

const input: Cups = process.argv[2].split('').map(Number);

pipeline(
	play(input),
	take(100),
	last,
	answer => {
		const cup1Index = answer!.indexOf(1);
		console.log('Part 1:', [...answer!.slice(cup1Index + 1), ...answer!.slice(0, cup1Index)]);
	},
);

function* play(cups: Cups): Generator<Cups> {
	const lowest = Math.min(...cups), highest = Math.max(...cups);

	let currentCup = cups[0];
	while (true) {
		const picked: Cups = [];

		for (let _ = 0; _ < 3; _++) {
			const currentCupIndex = cups.indexOf(currentCup);
			if (currentCupIndex + 1 < cups.length) {
				picked.push(cups.splice(currentCupIndex + 1, 1)[0]);
			} else {
				picked.push(cups.shift()!);
			}
		}

		const dest = destination(currentCup - 1);
		cups.splice(dest + 1, 0, ...picked);
		currentCup = cups[(cups.indexOf(currentCup) + 1) % cups.length];
		yield cups;
	}

	function destination(label: number): number {
		if (label < lowest) {
			return destination(highest);
		}

		const index = cups.indexOf(label);
		return index !== -1 ? index : destination(label - 1);
	}
}
