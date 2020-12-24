import { modulo } from '../../lib';
import { count, not, pipe, pipeline } from '../../lib/fp';
import { collectToArray, filter, map, range } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

type Direction = 'e' | 'se' | 'sw' | 'w' | 'nw' | 'ne';
type Coordinate = [number, number];

const ALL_DIRECTIONS: [Direction, Direction, Direction, Direction, Direction, Direction] = ['e' , 'se' , 'sw' , 'w' , 'nw' , 'ne'];
const REFERENCE: Coordinate = [0, 0];

const input = [...parseInput()];

{
	let currentIteration = new Set<string>();
	for (const directions of input) {
		const tile = toString(followDirections(directions));
		if (currentIteration.has(tile)) {
			currentIteration.delete(tile);
		} else {
			currentIteration.add(tile);
		}
	}
	console.log('Part 1:', currentIteration.size);


	const isBlackTile = (tile: Coordinate) => currentIteration.has(toString(tile));
	const countBlackTiles = pipe(filter(isBlackTile), count);

	for (const _ of range(100)) {
		const nextIteration = new Set<string>();

		for (const blackTile of map(fromString)(currentIteration)) {
			const adjacent = adjacentTiles(blackTile);

			if ([1, 2].includes(countBlackTiles(adjacent))) {
				nextIteration.add(toString(blackTile));
			}

			for (const whiteNeighborTile of filter(not(isBlackTile))(adjacent)) {
				if (pipeline(whiteNeighborTile, adjacentTiles, countBlackTiles) === 2) {
					nextIteration.add(toString(whiteNeighborTile));
				}
			}
		}

		currentIteration = nextIteration;
	}
	console.log('Part 2:', currentIteration.size);
}

function parseInput(): IterableIterator<Direction[]> {
	return pipeline(
		readBlocksFromStdin(),
		map(pipe(parseLine, collectToArray)),
	);
}

function* parseLine(line: string): IterableIterator<Direction> {
	for (let i = 0; i < line.length; ) {
		const length = ['e', 'w'].includes(line.charAt(i)) ? 1 : 2;
		yield line.slice(i, i + length) as Direction;
		i += length;
	}
}

function followDirections(directions: Direction[]): Coordinate {
	return directions.reduce(next, REFERENCE);
}

type Neighbors = [Coordinate, Coordinate, Coordinate, Coordinate, Coordinate, Coordinate];
function adjacentTiles(coordinate: Coordinate): Neighbors {
	return ALL_DIRECTIONS.map(direction => next(coordinate, direction)) as Neighbors;
}

function next([x , y]: Coordinate, direction: Direction): Coordinate {
	const xOffset = modulo(y, 2);
	switch (direction) {
	case 'e':  return [x + 1, y];
	case 'w':  return [x - 1, y];
	case 'ne': return [x + xOffset,     y + 1];
	case 'se': return [x + xOffset,     y - 1];
	case 'nw': return [x + xOffset - 1, y + 1];
	case 'sw': return [x + xOffset - 1, y - 1];
	}
}

function toString(coordinate: Coordinate): string {
	return JSON.stringify(coordinate);
}

function fromString(stringified: string): Coordinate {
	return JSON.parse(stringified);
}
