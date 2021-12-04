import { join, map, pipe, transpose } from 'ramda';
import { readBlocksFromStdin } from '../lib/index.js';

const report = [...readBlocksFromStdin()]
	.map(row => row.split('').map(Number));

const bitsPerReportValue = report[0].length;

{
	const gamma = gammaRating();
	const epsilon = epsilonRating();
	const powerConsumption = gamma * epsilon;
	const oxygen = oxygenRating();
	const co2Scrubber = co2ScrubberRating();
	const lifeSupport = oxygen * co2Scrubber;

	console.log('Gamma', gamma);
	console.log('Epsilon', epsilon)
	console.log('Power consumption', powerConsumption)
	console.log('Oxygen', oxygen);
	console.log('CO2 scrubber', co2Scrubber);
	console.log('Life support', lifeSupport);
}


function gammaRating() {
	return pipe(
		transpose,
		map(mostCommon),
		join(''),
		x => Number.parseInt(x, 2),
	)(report);
}

function epsilonRating() {
	return ~gammaRating() & ((1 << bitsPerReportValue) - 1);
}

function lifeSupportRating(filterFn) {
	let values = [...report];
	for (let i = 0; i < bitsPerReportValue && values.length > 1; i++) {
		const filterValue = filterFn(column(i, values));
		values = values.filter(row => row[i] === filterValue);
	}
	return Number.parseInt(values[0].join(''), 2);
}

function oxygenRating() { return lifeSupportRating(mostCommon); }
function co2ScrubberRating() { return lifeSupportRating(leastCommon); }

function mostCommon(array) {
	const count = countBits(array);
	return count[1] >= count[0] ? 1 : 0;
}

function leastCommon(array) {
	return Number(!mostCommon(array));
}

function countBits(array) {
	let count = [0, 0];
	for (const bit of array) {
		count[bit]++;
	}
	return count;
}

function column(n, table) {
	return table.map(row => row[n]);
}
