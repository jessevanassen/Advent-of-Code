import { add } from 'ramda';
import { readBlocksFromStdin } from '../lib/index.js';


const input = [...readBlocksFromStdin()];

console.log(
	'Part 1',
	input
		.map(findCorrupted)
		.filter(Boolean)
		.map(c => ({
			')': 3,
			']': 57,
			'}': 1197,
			'>': 25137,
		})[c])
		.reduce(add, 0)
);

{
	const scores = input
			.map(removeBalancedGroups)
			.filter(line => !findCorrupted(line))
			.map(line => [...line]
					.map(c => ({
						'(': 1,
						'[': 2,
						'{': 3,
						'<': 4,
					})[c])
					.reduceRight((total, score) => total * 5 + score, 0))
			.sort((a, b) => a - b);
	console.log('Part 2', scores[(scores.length - 1) / 2]);
}


function findCorrupted(line) {
	return removeBalancedGroups(line)
			.replace(/\(|\[|\{|\</g, '')
			[0];
}

function removeBalancedGroups(line) {
	while (true) {
		const next = line.replace(/\(\)|\[\]|\{\}|\<\>/g, '');
		if (next === line) {
			return next;
		}
		line = next;
	}
}
