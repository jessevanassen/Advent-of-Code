import { readBlocksFromStdin } from '../../lib/fs';
import { split, map, filter, collectToArray, product, range } from '../../lib/fp/generators';
import { eq, count, pipe, truthy } from '../../lib/fp';

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
	filter<string>(truthy),
	map(pipe(
		split(),
		map(eq('#')),
		collectToArray,
	)),
	collectToArray,
)(readBlocksFromStdin());

const treeCountsOfSlopes = pipe(
	map(treeCount(trees)),
	collectToArray,
)(slopes);

console.log(`Encountered ${treeCountsOfSlopes[1]} for slope ${JSON.stringify(slopes[1])}`);
console.log(`Each listed slope multiplied: ${product(treeCountsOfSlopes)}`);

function treeCount(trees: Area) {
	return ({ right, down }: Slope) =>
		pipe(
			filter(containsTree(trees, right)),
			count,
		)(range(trees.length, { step: down }));
}

function containsTree(trees: Area, indexFactor: number) {
	return (index: number) => trees[index][(index * indexFactor) % trees[index].length];
}
