import { log } from '../../lib';
import { pipe } from '../../lib/fp';
import { collectToArray, filter, first, map, min, range, split } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

type Bus = number | 'x';

const { departure, buses } = parseInput();

pipe(
	filter(notOutOfService),
	map(bus => [bus, nextDeparture(departure, bus)]),
	min(([, nextDeparture]) => nextDeparture),
	([bus, nextDeparture]) => bus * (nextDeparture - departure),
	log('Part 1:'),
)(buses);

function nextDeparture(timestamp: number, bus: number) {
	return Math.ceil(timestamp / bus) * bus;
}

function notOutOfService(bus: Bus): bus is Exclude<Bus, 'x'> {
	return bus !== 'x';
}

function parseInput() {
	const [firstLine, secondLine] = [...readLinesFromStdin()];
	const departure = Number.parseInt(firstLine);
	const buses: Bus[] = pipe(
		split(','),
		map(bus => bus === 'x' ? 'x' : Number.parseInt(bus)),
		collectToArray,
	)(secondLine);

	return { departure, buses };
}
