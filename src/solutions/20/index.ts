import { remove } from '../../lib';
import { eq, pipe } from '../../lib/fp';
import { collectToArray, map, range, zipWithIndex } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

const TILE_SIZE = 10;

const seaMonster = [
	'                  # ',
	'#    ##    ##    ###',
	' #  #  #  #  #  #   ',
]
	.map(line => line
		.split('')
		.map(eq('#')));
const seamonsterSize = seaMonster
	.flat()
	.filter(Boolean)
	.length;

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

interface Image {
	size: number;
	get(x: number, y: number): boolean;
}

const tiles = parseTiles();
const imageSize = Math.sqrt(tiles.length);

const configurations = validConfigurations(tiles);
console.log('Part 1:', configurations[0][0].id * configurations[0][imageSize - 1].id * configurations[0][tiles.length - imageSize].id * configurations[0][tiles.length - 1].id);

for (const configuration of configurations) {
	const img = Image(configuration);
	const count = countSeaMonsters(img);

	if (count > 0) {
		console.log('Part 2: ', countTrue(img) - count * seamonsterSize);
	}
}

function validConfigurations(tiles: Tile[]): Tile[][] {
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

		const x = acc.length % imageSize,
			y = ~~(acc.length / imageSize);

		const result: Tile[][] = [];

		for (const [next, tileIndex] of zipWithIndex(deck)) {
			for (const nextConfiguration of tileVariants.get(next)!) {
				const { left, top } = variantBorders.get(nextConfiguration)!;
				if (
					(x === 0 || left === variantBorders.get((acc[y * imageSize + (x - 1)]))!.right) &&
					(y === 0 || top ===  variantBorders.get((acc[(y - 1) * imageSize + x]))!.bottom)
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

		const rowBytes = new Uint16Array(TILE_SIZE);
		for (let y = 0; y < TILE_SIZE; y++) {
			for (let x = 0; x < TILE_SIZE; x++) {
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
	const indices = [...range(TILE_SIZE)];

	return {
		top: toNumber(indices.map(x => tile.get(x, 0))),
		right: toNumber(indices.map(y => tile.get(TILE_SIZE - 1, y))),
		bottom: toNumber(indices.map(x => tile.get(x, TILE_SIZE - 1))),
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
			const h = ((TILE_SIZE - 1) / 2);
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
			return get(TILE_SIZE - 1 - x, y);
		},
		id,
	};
}

function Image(tiles: Tile[]): Image {
	const size = TILE_SIZE - 2;
	return {
		size: size * imageSize,
		get(x, y) {
			const i = ~~(x / size) + ~~(y / size) * imageSize;
			const tile = tiles[i];
			return tile.get(
				(x % size) + 1,
				(y % size) + 1);
		},
	};
}

function countSeaMonsters(image: Image): number {
	let result = 0;

	for (const y of range(image.size - seaMonster.length)) {
		for (const x of range(image.size - seaMonster[0].length)) {
			if ([...range(seaMonster.length)]
				.every(_y => [...range(seaMonster[0].length)]
					.every(_x => !seaMonster[_y][_x] || image.get(x + _x, y + _y)))) {
				result++;
			}
		}
	}

	return result;
}

function countTrue(image: Image): number {
	let result = 0;
	for (const x of range(image.size)) {
		for (const y of range(image.size)) {
			if (image.get(x, y)) {
				result++;
			}
		}
	}
	return result;
}
