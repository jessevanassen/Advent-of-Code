import { readLinesFromStdin } from '../../lib/fs';
import { split, map, filter, collect, product, range, } from '../../lib/fp/generators';
import { eq, length, pipe, truthy } from '../../lib/fp';

interface Slope {
	right: number;
	down: number;
}

type Area = boolean[][];

const slopes: Slope[] = [
	{ right: 1, down: 1 },
	{ right: 3, down: 1 },
	{ right: 5, down: 1 },
	{ right: 7, down: 1 },
	{ right: 0.5, down: 2 },
];

const trees: Area = pipe(
	filter(truthy),
	map(pipe(
		split(),
		map(eq('#')),
		collect,
	)),
	collect,
)(readLinesFromStdin());

const treeCountsOfSlopes = pipe(
	map(treeCount(trees)),
	collect,
)(slopes);

console.log(`Encountered ${treeCountsOfSlopes[1]} for slope ${JSON.stringify(slopes[1])}`);
console.log(`Each listed slope multiplied: ${product(treeCountsOfSlopes)}`);

function treeCount(trees: Area) {
	return ({ right, down }: Slope) => 
		pipe(
			filter(containsTree(trees, right)),
			length,
		)(range(trees.length, { step: down }));
}

function containsTree(trees: Area, indexFactor: number) {
	return (index: number) => trees[index][(index * indexFactor) % trees[index].length];
}
