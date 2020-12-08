import { seatId } from './binary-boarding.js';
import { pipe, truthy } from '../../lib/fp';
import { readLinesFromStdin } from '../../lib/fs/index.js';
import { collectToArray, filter, map } from '../../lib/fp/generators/index.js';

const seats = pipe(
	filter(truthy),
	map(seatId),
	collectToArray,
)(readLinesFromStdin());

const min = Math.min(...seats),
	max = Math.max(...seats);

console.log(`Highest seat ID: ${max}`);

for (let i = min + 1; i < max; i++) {
	if (!seats.includes(i)) {
		console.log(`Missing seat ID: ${i}`);
		break;
	}
}
