import { log } from '../../lib';
import { pipe } from '../../lib/fp';
import { collectToArray, filter, map, min, skip, split } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

type Bus = number | 'x';

const { departure, buses } = parseInput();

pipe(
	filter(notOutOfService),
	map(bus => [bus, nextDeparture(departure, bus)]),
	min(([, nextDeparture]) => nextDeparture),
	([bus, nextDeparture]) => bus * (nextDeparture - departure),
	log('Part 1:'),
)(buses);

const ranges: [number, number][] = pipe(
	(buses: Bus[]) => buses.map((bus, index) => [bus, index]),
	filter((busSchedule): busSchedule is [number, number] => notOutOfService(busSchedule[0])),
	collectToArray,
)(buses);

let time = ranges[0][0];
let factor = time;

for (const [bus, offset] of skip(1)(ranges)) {
	while ((time + offset) % bus !== 0) {
		time += factor;
	}
	factor *= bus;
}

log('Part 2:')(time);

function nextDeparture(timestamp: number, bus: number) {
	return Math.ceil(timestamp / bus) * bus;
}

function notOutOfService(bus: Bus): bus is Exclude<Bus, 'x'> {
	return bus !== 'x';
}

function parseInput() {
	const [firstLine, secondLine] = [...readBlocksFromStdin()];
	const departure = Number.parseInt(firstLine);
	const buses: Bus[] = pipe(
		split(','),
		map(bus => bus === 'x' ? 'x' : Number.parseInt(bus)),
		collectToArray,
	)(secondLine);

	return { departure, buses };
}
