/**
 * @typedef {{ xMin: number; xMax: number; yMin: number; yMax: number }} Target
 */
import { readFromStdin, sumRange, takeWhile, Vector2 } from '../lib/index.js';

const target = parseInput();

let highest = { y: 0, initialVelocity: Vector2() };
let count = 0;
for (const x of possibleX(target)) {
	for (const y of possibleY(target)) {
		const trajectory = [...takeWhile(
			notBelow(target),
			fireProbe({ x, y }),
		)];

		if (!trajectory.some(p => within(target, p))) { continue; }

		count++;

		const maxY = trajectory.map(({ y }) => y).reduce((x, y) => Math.max(x, y));

		if (maxY > highest.y) {
			highest = { y: maxY, initialVelocity: Vector2({ x, y })}
		}
	}
}

console.log('Part 1', highest.y);
console.log('Part 2', count);

/**
 * @param {Vector2} velocity
 * @returns {Generator<Vector2>}
 */
function* fireProbe(velocity) {
	const xRange = xTrajectory(velocity.x);
	const yRange = yTrajectory(velocity.y);

	while (true) {
		yield Vector2({
			x: xRange.next().value,
			y: yRange.next().value,
		});
	}
}

/**
 * @param {number} xVelocity
 * @returns {Generator<number>}
 */
function* xTrajectory(xVelocity) {
	let xPosition = 0;
	while (true) {
		yield xPosition;
		xPosition += xVelocity;
		xVelocity = Math.max(0, xVelocity - 1);
	}
}

/**
 * @param {number} yVelocity
 * @returns {Generator<number>}
 */
function* yTrajectory(yVelocity) {
	let yPosition = 0;
	while (true) {
		yield yPosition;
		yPosition += yVelocity;
		yVelocity -= 1;
	}
}

/**
 * @param {Target} target
 * @param {Vector2} position
 * @returns {boolean}
 */
function within({ xMin, xMax, yMin, yMax }, { x, y }) {
	return (xMin <= x && x <= xMax) && (yMin <= y && y <= yMax);
}

/**
 * @param {Target} target
 * @returns {(position: Vector2) => boolean}
 */
function notBelow({ yMin }) {
	return ({ y }) => y >= yMin;
}

/**
 * @param {Target} target
 * @returns {Generator<number>}
 */
function* possibleX({ xMin, xMax }) {
	for (let x = 1; x <= xMax; x++) {
		if (sumRange(x) < xMin) { continue; }
		yield x;
	}
}

/**
 * @param {Target} target
 * @returns {Generator<number>}
 */
function* possibleY({ yMin }) {
	for (let y = yMin; y < 1000; y++) {
		yield y;
	}
}

function parseInput() {
	const pattern = /target area: x=(?<xMin>-?\d+)..(?<xMax>-?\d+), y=(?<yMin>-?\d+)..(?<yMax>-?\d+)/
	const input = readFromStdin();
	const { xMin, xMax, yMin, yMax } = input.match(pattern).groups;
	return {
		xMin: Number(xMin),
		xMax: Number(xMax),
		yMin: Number(yMin),
		yMax: Number(yMax),
	};
}
