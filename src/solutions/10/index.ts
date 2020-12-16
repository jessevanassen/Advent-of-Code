import { isBetween, log } from '../../lib';
import { numberComparator, pipe, sort } from '../../lib/fp';
import { aperture, map, product, reduce } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

const inputs: number[] = pipe(
	map(Number),
	sort(numberComparator),
)(readLinesFromStdin());

const deviceRating = inputs[inputs.length - 1] + 3;

pipe(
	() => [0, ...inputs, deviceRating],
	aperture,
	map(([x, y]) => y - x),
	reduce(([one, three], difference) => [one + Number(difference === 1), three + Number(difference === 3)], [0, 0]),
	product,
	log('Part 1:'),
)(null);

log('Part 2')(possibleArrangements(inputs));

function possibleArrangements(adapters: number[]): number {
	const cache: Record<number, number> = {};

	function _arrangements(startIndex: number): number {
		if (startIndex >= adapters.length - 1) {
			return 1;
		}

		if (startIndex in cache) {
			return cache[startIndex];
		}

		let acc = 0;
		for (let i = startIndex + 1; adapterCompatibleWith(adapters[startIndex] ?? 0)(adapters[i]); i++) {
			acc += _arrangements(i);
		}
		return cache[startIndex] = acc;
	}

	return _arrangements(-1);
}


function adapterCompatibleWith(joltage: number) {
	return isBetween(joltage + 1, joltage + 3);
}
