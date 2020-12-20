import { remove } from "../../lib";
import { pipe } from "../../lib/fp";
import { collectToArray, flatMap, map, range, zipWithIndex } from "../../lib/fp/generators";
import { readBlocksFromStdin } from "../../lib/fs";

const SIZE = 10;

interface Tile {
	id: number;
	get(x: number, y: number): boolean;
}

interface TileBorders {
	top: number;
	right: number;
	bottom: number;
	left: number;
}

const tiles = parseTiles();

for (const possibleGrid of possibleGrids(tiles)) {
	const size = imageSize(tiles);
	console.log("Part 1:", possibleGrid[0][1] * possibleGrid[size - 1][1] * possibleGrid[tiles.length - size][1] * possibleGrid[tiles.length - 1][1]);
}

type TileConfiguration = [TileBorders, number];
function possibleGrids(tiles: Tile[]): TileConfiguration[][] {
	const size = imageSize(tiles);
	const tileBorderVariants = tiles
		.reduce((acc, tile) => {
			acc[tile.id] = variants(TileBorders(tile));
			return acc;
		}, {} as { [id: number]: TileBorders[] });

	function _possibleGrids(deck: Tile[], acc: TileConfiguration[]): TileConfiguration[][] {
		if (deck.length === 0) {
			return [acc];
		}


		const x = acc.length % size,
			y = ~~(acc.length / size);

		const result: TileConfiguration[][] = [];

		for (const [next, tileIndex] of zipWithIndex(deck)) {
			for (const nextConfiguration of tileBorderVariants[next.id]) {
				if (
					(x === 0 || nextConfiguration.left === acc[y * size + (x - 1)][0].right) &&
					(y === 0 || nextConfiguration.top ===  acc[(y - 1) * size + x][0].bottom)
				) {
					result.push(..._possibleGrids(remove(tileIndex, deck), [...acc, [nextConfiguration, next.id]]));
				}
			}
		}

		return result;
	}

	return _possibleGrids(tiles, []);
}

function parseTiles(): Tile[] {
	return pipe(
		map(parseTile),
		collectToArray,
	)(readBlocksFromStdin('\n\n'));

	function parseTile(tileData: string): Tile {
		const [header, ...rows] = tileData.split('\n');
		const id = Number.parseInt(header.match(/^Tile (\d+):$/)![1]);

		const rowBytes = new Uint16Array(SIZE);
		for (let y = 0; y < SIZE; y++) {
			for (let x = 0; x < SIZE; x++) {
				rowBytes[y] |= (rows[y].charAt(x) === '#' ? 1 : 0) << x;
			}
		}

		return {
			id,
			get: (x, y) => (rowBytes[y] & (1 << x)) !== 0,
		};
	}
}

function TileBorders(tile: Tile): TileBorders {
	const indices = [...range(SIZE)];

	return {
		top: toNumber(indices.map(x => tile.get(x, 0))),
		right: toNumber(indices.map(y => tile.get(SIZE - 1, y))),
		bottom: toNumber(indices.map(x => tile.get(x, SIZE - 1))),
		left: toNumber(indices.map(y => tile.get(0, y))),
	}

	function toNumber(bools: boolean[]): number {
		return bools.reduce((acc, x) => acc << 1 | Number(x), 0);
	}
}

function variants(tileBorders: TileBorders): TileBorders[] {
	const result = [tileBorders, flipHorizontal(tileBorders)];
	for (let _ = 0; _ < 3; _++) {
		result.push(rotateCW(result[result.length - 2]));
		result.push(rotateCW(result[result.length - 2]));
	}
	return result;
}

function rotateCW(tileBorders: TileBorders): TileBorders {
	return {
		top: reverse(tileBorders.left),
		right: tileBorders.top,
		bottom: reverse(tileBorders.right),
		left: tileBorders.bottom,
	}
}

function flipHorizontal(tileBorders: TileBorders): TileBorders {
	return {
		top: reverse(tileBorders.top),
		right: tileBorders.left,
		bottom: reverse(tileBorders.bottom),
		left: tileBorders.right,
	}
}

function reverse(number: number): number {
	let out = 0;
	for (let i = 0; i < SIZE; i++) {
		out = (out << 1) | ((number >> i) & 1);
	}
	return out;
}

function imageSize(tiles: Tile[]): number {
	return Math.sqrt(tiles.length);
}

function toString(tile: Tile): string {
	const rows: string[] = [];
	for (let y = 0; y < SIZE; y++) {
		rows.push('');
		for (let x = 0; x < SIZE; x++) {
			rows[y] = rows[y] + (tile.get(x, y) ? '#' : '.');
		}
	}
	return `Tile ${tile.id}:\n${rows.join('\n')}`;
}
