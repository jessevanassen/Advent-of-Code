import { partition, range, repeat, scan, uniqBy } from 'ramda';
import { readFromStdin, Vector2 } from '../lib/index.js';

const { dots, folds } = parseInput();
const afterFolds = scan(applyFold, dots, folds);

console.log('Part 1: %d', afterFolds[0].length);
console.log('Part 2: \n%s', toString(afterFolds.at(-1)));

/**
 * @param {Vector2[]} coordinates
 * @param {Vector2} foldLine
 * @returns {Vector2[]}
 */
function applyFold(coordinates, foldLine) {
	const [beforeFold, afterFold] = partition(
			c => foldLine.x === 0 ?
					c.y <= foldLine.y :
					c.x <= foldLine.x,
			coordinates);

	const afterFoldApplied = afterFold
			.map(v => Vector2.subtract(v, foldLine))
			.map(v => Vector2.multiply(v,
					foldLine.x === 0 ?
							{ x:  1,  y: -1 } :
							{ x: -1,  y:  1 }))
			.map(v => Vector2.add(v, foldLine))

	return uniqBy(Vector2.toString, [...beforeFold, ...afterFoldApplied]);
}

function parseInput() {
	const [dots, folds] = readFromStdin().split('\n\n');

	return {
		dots: dots.split('\n').map(Vector2.parse),
		folds: folds.split('\n')
				.map(fold => fold.match(/fold along ([xy])=(\d+)/))
				.map(([/**/, axis, c]) => axis === 'x' ?
						Vector2({ x: Number(c) }) :
						Vector2({ y: Number(c) })),
	};
}

/**
 * @param {Vector2[]} dots
 * @returns {string}
 */
function toString(dots) {
	const maxX = Math.max(...dots.map(({ x }) => x));
	const maxY = Math.max(...dots.map(({ y }) => y));

	const output = range(0, maxY + 2)
			.map(_ => repeat(' ', maxX + 1));

	for (const { x, y } of dots) {
		output[y][x] = '#';
	}

	return output.map(l => l.join('')).join('\n');
}
