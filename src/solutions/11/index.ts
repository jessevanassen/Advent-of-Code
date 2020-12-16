import { count, eq, not, pipe } from '../../lib/fp';
import { aperture, collectToArray, filter, first, flatten, iterate, map, skipWhile, split, sum, takeWhile } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

type Vector = [number, number];
const directions: Vector[] = [
	[-1, -1], [-1, 0], [-1, 1],
	[ 0, -1], [ 0, 1],
	[ 1, -1], [ 1, 0], [ 1, 1]];

type GridPosition = '.' | 'L' | '#';
type Grid = GridPosition[][];

const grid: Grid = pipe(
	map(pipe(
		split(),
		filter(isValidGridPosition),
		collectToArray,
	)),
	collectToArray,
)(readLinesFromStdin());

console.log('Part 1:', pipe(evolveUntilStable(EvolutionFunction(countAdjacentOccupied, 4)), countOccupied)(grid));
console.log('Part 2:', pipe(evolveUntilStable(EvolutionFunction(countVisibleOccupied, 5)), countOccupied)(grid));

type EvolutionFunction = (x: number, y: number, grid: Grid) => GridPosition;

function evolveUntilStable(fn: EvolutionFunction) {
	return pipe(
		iterate(evolve(fn)),
		aperture,
		skipWhile(grids => !gridsEqual(...grids)),
		first,
		x => x![0],
	);
}

function evolve(fn: EvolutionFunction) {
	return function(grid: Grid): Grid {
		const next: Grid = [];
		for (let y = 0; y < grid.length; y++) {
			const row: GridPosition[] = [];
			for (let x = 0; x < grid[y].length; x++) {
				row.push(fn(x, y, grid));
			}
			next.push(row);
		}
		return next;
	};
}

function EvolutionFunction(adjacentFunction: (x: number, y: number, grid: Grid) => number, minimumOccupied: number) {
	return function(x: number, y: number, grid: Grid): GridPosition {
		const position = grid[y][x];
		if (position === '.')
			return '.';

		const adjacentOccupied = adjacentFunction(x, y, grid);

		if (position === 'L' && adjacentOccupied === 0)
			return '#';

		if (position === '#' && adjacentOccupied >= minimumOccupied)
			return 'L';

		return position;
	};
}


function countAdjacentOccupied(x: number, y: number, grid: Grid): number {
	return pipe(
		map(pipe(
			addVector([x, y]),
			([x, y]) => grid[y]?.[x],
			eq('#'),
			Number,
		)),
		sum,
	)(directions);
}

function countVisibleOccupied(x: number, y: number, grid: Grid): number {
	return pipe(
		map(pipe(
			visible(x, y, grid),
			filter(not(eq('.'))),
			first,
			eq('#'),
			Number,
		)),
		sum,
	)(directions);
}

function visible(x: number, y: number, grid: Grid) {
	return (direction: Vector) => pipe(
		iterate(addVector(direction)),
		map(([x, y]) => grid[y]?.[x]),
		takeWhile(not(eq(undefined))),
	)([x, y]);
}

function isValidGridPosition(position: string): position is GridPosition {
	return ['.', 'L', '#'].includes(position);
}

function gridsEqual(x: Grid, y: Grid): boolean {
	for (let i = 0; i < x.length; i++)
		for (let j = 0; j < x[i].length; j++)
			if (x[i][j] !== y[i][j])
				return false;
	return true;
}

function countOccupied(grid: Grid) {
	return pipe(
		flatten,
		filter(eq('#')),
		count,
	)(grid);
}

function addVector(v1: Vector) {
	return (v2: Vector): Vector => [
		v1[0] + v2[0],
		v1[1] + v2[1],
	];
}
