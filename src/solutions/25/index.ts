import { log } from '../../lib';
import { pipeline } from '../../lib/fp';
import { first, filter, map, collectToArray, zipWithIndex, skip, iterate } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

const SUBJECT_NUMBER = 7;
const DIVIDER = 20201227;

const keys = pipeline(
	readBlocksFromStdin(),
	map(Number),
	collectToArray,
) as [number, number];


function values(subjectNumber: number): IterableIterator<number> {
	return iterate((value: number) => (value * subjectNumber) % DIVIDER)(1);
}

function findLoopSize(key: number): number {
	return pipeline(
		values(SUBJECT_NUMBER),
		zipWithIndex,
		filter(([value]) => value === key),
		first,
		result => result![1] + 1,
	);
}

pipeline(
	values(keys[0]),
	skip(findLoopSize(keys[1]) - 1),
	first,
	log('Part 1:'),
);
