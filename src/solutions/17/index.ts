import { count, pipe } from '../../lib/fp';
import { filter, map, range, reduce, zipWithIndex } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';
import { addScalar, equals, fromString, max, min, toString, Vector4 } from '../../lib/vector/vector4';

const initialSpace: Set<string> = parseInput();

for (const clampW of [true, false]) {
	const space = reduce(acc => evolve(acc, clampW), initialSpace)(range(6));
	console.log(`Part ${clampW ? 1 : 2}:`, space.size);
}

function evolve(space: Set<string>, clampW: boolean): Set<string> {
	const next = new Set<string>();

	const [min, max] = minmax(space);
	for (const coordinate of coordinatesBetween(addScalar(-1)(min), addScalar(1)(max))) {
		if (clampW && coordinate[3] !== 0) continue;

		const activeNeighborCount = pipe(
			neighbors,
			filter(c => space.has(toString(c))),
			count,
		)(coordinate);

		if (activeNeighborCount === 3 || (activeNeighborCount === 2 && space.has(toString(coordinate)))) {
			next.add(toString(coordinate));
		}
	}

	return next;
}

function parseInput(): Set<string> {
	const result = new Set<string>();
	for (const [row, y] of zipWithIndex(readLinesFromStdin())) {
		for (const [entry, x] of zipWithIndex(row)) {
			if (entry === '#') {
				result.add(toString([x, y, 0, 0]));
			}
		}
	}
	return result;
}

function neighbors(vector: Vector4): IterableIterator<Vector4> {
	return filter((x: Vector4) => !equals(x, vector))(
		coordinatesBetween(addScalar(-1)(vector), addScalar(1)(vector)));
}

function* coordinatesBetween(x: Vector4, y: Vector4): IterableIterator<Vector4> {
	for (const _x of range(y[0] + 1, { start: x[0]})) {
		for (const _y of range(y[1] + 1, { start: x[1]})) {
			for (const _z of range(y[2] + 1, { start: x[2]})) {
				for (const _w of range(y[3] + 1, { start: x[3]})) {
					yield [_x, _y, _z, _w];
				}
			}
		}
	}
}

function minmax(space: Set<string>) {
	return pipe(
		map(fromString),
		reduce(
			(acc: [Vector4, Vector4], x: Vector4): [Vector4, Vector4] => [min(acc[0], x), max(acc[1], x)],
			[[Infinity, Infinity, Infinity, Infinity], [-Infinity, -Infinity, -Infinity, -Infinity]]),
	)(space.keys());
}
