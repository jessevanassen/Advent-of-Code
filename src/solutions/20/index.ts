import { remove } from '../../lib';
import { pipe } from '../../lib/fp';
import { collectToArray, map, range, zipWithIndex } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

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

const configurations = validConfigurations(tiles);
{
	const size = imageSize(tiles);
	console.log('Part 1:', configurations[0][0].id * configurations[0][size - 1].id * configurations[0][tiles.length - size].id * configurations[0][tiles.length - 1].id);
}

function validConfigurations(tiles: Tile[]): Tile[][] {
	const size = imageSize(tiles);

	const tileVariants = new Map<Tile, Tile[]>(
		tiles.map(tile => [tile, variants(tile)]));
	const variantBorders = new Map<Tile, TileBorders>();
	for (const variants of tileVariants.values()) {
		for (const variant of variants) {
			variantBorders.set(variant, borders(variant));
		}
	}

	function _possibleGrids(deck: Tile[], acc: Tile[]): Tile[][] {
		if (deck.length === 0) {
			return [acc];
		}

		const x = acc.length % size,
			y = ~~(acc.length / size);

		const result: Tile[][] = [];

		for (const [next, tileIndex] of zipWithIndex(deck)) {
			for (const nextConfiguration of tileVariants.get(next)!) {
				const { left, top } = variantBorders.get(nextConfiguration)!;
				if (
					(x === 0 || left === variantBorders.get((acc[y * size + (x - 1)]))!.right) &&
					(y === 0 || top ===  variantBorders.get((acc[(y - 1) * size + x]))!.bottom)
				) {
					result.push(..._possibleGrids(remove(tileIndex, deck), [...acc, nextConfiguration]));
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

function borders(tile: Tile): TileBorders {
	const indices = [...range(SIZE)];

	return {
		top: toNumber(indices.map(x => tile.get(x, 0))),
		right: toNumber(indices.map(y => tile.get(SIZE - 1, y))),
		bottom: toNumber(indices.map(x => tile.get(x, SIZE - 1))),
		left: toNumber(indices.map(y => tile.get(0, y))),
	};

	function toNumber(bools: boolean[]): number {
		return bools.reduce((acc, x) => acc << 1 | Number(x), 0);
	}
}

function variants(tile: Tile): Tile[] {
	const result = [tile, flipHorizontal(tile)];
	for (let _ = 0; _ < 3; _++) {
		result.push(rotateCW(result[result.length - 2]));
		result.push(rotateCW(result[result.length - 2]));
	}
	return result;
}

function rotateCW({ id, get }: Tile): Tile {
	return {
		get(x, y) {
			const h = ((SIZE - 1) / 2);
			const _x = (y - h) * -1 + h;
			const _y = (x - h) + h;
			return get(_x, _y);
		},
		id,
	};
}

function flipHorizontal({ id, get }: Tile): Tile {
	return {
		get(x, y) {
			return get(SIZE - 1 - x, y);
		},
		id,
	};
}

function imageSize(tiles: Tile[]): number {
	return Math.sqrt(tiles.length);
}
