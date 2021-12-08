import { add, allPass, filter, length, map, pipe, sum } from 'ramda';
import { readBlocksFromStdin } from '../lib/index.js';


const input = [...readBlocksFromStdin()].map(line => line
			.split(' | ', 2)
			.map(part => part
					.split(' ')
					.map(part => part
							.split('')
							.sort()
							.join(''))));


console.log('Part 1', solvePart1());
console.log('Part 2', solvePart2());


function solvePart1() {
	return pipe(
		map(line => line[1]),
		map(filter(digit => [2, 3, 4, 7].includes(digit.length))),
		map(length),
		sum,
	)(input);
}

function solvePart2() {
	return input
			.map(line => deriveDigits(line[0], line[1]))
			.reduce(add);
}

/**
 * @param {string[]} checks
 * @param {string[]} input
 * @returns {number}
 */
function deriveDigits(checks, input) {
	const segments = [];

	const not = (...digits) => segment => !digits.some(d => segment === segments[d]);
	const segmentCount = length => x => x.length === length;
	const contains = digit => segment => [...segments[digit]].every(x => segment.includes(x));
	const isContainedBy = digit => segment => [...segment].every(x => segments[digit].includes(x));
	const deduct = (...predicates) => checks.find(allPass(predicates));
	const toDigit = segment => segments.indexOf(segment);

	segments[1] = deduct(segmentCount(2));
	segments[4] = deduct(segmentCount(4));
	segments[7] = deduct(segmentCount(3));
	segments[8] = deduct(segmentCount(7));
	segments[9] = deduct(segmentCount(6), contains(4));
	segments[0] = deduct(segmentCount(6), not(9), contains(7));
	segments[6] = deduct(segmentCount(6), not(0, 9));
	segments[5] = deduct(segmentCount(5), isContainedBy(6));
	segments[3] = deduct(segmentCount(5), not(5), contains(7));
	segments[2] = deduct(segmentCount(5), not(3, 5));

	return Number.parseInt(input.map(toDigit).join(''), 10);
}

