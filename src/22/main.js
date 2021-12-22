/**
 * @typedef {{ command: boolean; block: Block }} Command
 * @typedef {{ from: Vector3, to: Vector3 }} Block
 */

import { readBlocksFromStdin, Vector3 } from '../lib/index.js';

const input = parseInput();

/** @type {Block} */
const initializationProcedureArea = { from: Vector3({ x: -50, y: -50, z: -50 }), to: Vector3({ x: 51, y: 51, z: 51 })};

const blocks = input.reduce(step, []);
const withoutInitializationProcedureArea = blocks.flatMap(cube =>
		difference(cube, initializationProcedureArea));

console.log('Part 1', size(blocks) - size(withoutInitializationProcedureArea));
console.log('Part 2', size(blocks));

/**
 *
 * @param {Block[]} blocks
 * @param {Command} command
 * @returns {Block[]}
 */
function step(blocks, { command, block }) {
	if (command) {
		const toAdd = blocks.reduce((blocksToAdd, existingBlock) =>
				blocksToAdd.flatMap(b => difference(b, existingBlock)), [block]);
		return [
			...blocks,
			...toAdd,
		];
	} else {
		return blocks.flatMap(b => difference(b, block));
	}
}

/**
 * @param {Block} block1
 * @param {Block} block2
 * @returns {Block[]}
 */
function difference(block1, block2) {
	const intersect = intersection(block1, block2);
	if (!intersect) {
		return [block1];
	}

	/** @type {Block[]} */
	const subBlocks = [
		{
			from: block1.from,
			to:   Vector3({ ...block1.to,   x: intersect.from.x })
		},
		{
			from: Vector3({ ...block1.from, x: intersect.to.x }),
			to: block1.to
		},

		{
			from: Vector3({ x: intersect.from.x, y: block1.from.y,    z: block1.from.z }),
			to:   Vector3({ x: intersect.to.x,   y: intersect.from.y, z: block1.to.z }),
		},
		{
			from: Vector3({ x: intersect.from.x, y: intersect.to.y, z: block1.from.z }),
			to:   Vector3({ x: intersect.to.x,   y: block1.to.y,    z: block1.to.z }),
		},

		{
			from: Vector3({ ...intersect.from, z: block1.from.z }),
			to:   Vector3({ ...intersect.to,   z: intersect.from.z }),
		},
		{
			from: Vector3({ ...intersect.from, z: intersect.to.z }),
			to:   Vector3({ ...intersect.to,   z: block1.to.z }),
		},
	];

	return subBlocks.filter(b => !isNegativeBlock(b));
}

/**
 * @param {Block[]} blocks
 */
function size(blocks) {
	return blocks
			.map(blockSize)
			.reduce((x, y) => x + y, 0);
}

/**
 * @param {Block} block
 * @returns {number}
 */
function blockSize({ from, to }) {
	return (to.x - from.x) * (to.y - from.y) * (to.z - from.z);
}

/**
 * @param {Block} block1
 * @param {Block} block2
 * @returns {Block | null}
 */
 function intersection(block1, block2) {
	const intersection = {
		from: {
			x: Math.max(block1.from.x, block2.from.x),
			y: Math.max(block1.from.y, block2.from.y),
			z: Math.max(block1.from.z, block2.from.z),
		},
		to: {
			x: Math.min(block1.to.x, block2.to.x),
			y: Math.min(block1.to.y, block2.to.y),
			z: Math.min(block1.to.z, block2.to.z),
		}
	}

	if (isNegativeBlock(intersection)) { return null; }

	return intersection;
}

/**
 * @param {Block} block
 */
function isNegativeBlock(block) {
	return block.from.x >= block.to.x ||
			block.from.y >= block.to.y ||
			block.from.z >= block.to.z;
}

/**
 * @returns {Command[]}
 */
function parseInput() {
	const pattern = /(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)/;
	return [...readBlocksFromStdin()]
		.map(line => line.match(pattern))
		.filter(Boolean)
		.map(([/**/, command, xStart, xEnd, yStart, yEnd, zStart, zEnd]) => ({
			command: command === 'on',
			block: {
				from: Vector3({
					x: Number(xStart),
					y: Number(yStart),
					z: Number(zStart),
				}),
				to: Vector3({
					x: Number(xEnd) + 1,
					y: Number(yEnd) + 1,
					z: Number(zEnd) + 1,
				}),
			},
		}));
}
