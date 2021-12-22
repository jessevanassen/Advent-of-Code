/**
 * @typedef {{ position: Vector3, beacons: Vector3[]}} Scanner
 * @typedef {{ points: [Vector3, Vector3], direction: Vector3 }} Fingerprint
 */

import { apply, compose, equals, partition, prop, repeat, xprod } from 'ramda';
import { combinations, filter, Interner, map, permutations, readFromStdin, setUtils, Vector3 } from '../lib/index.js';

const intern = Interner(Vector3.toString);

const transformationFunctions = (() => {
	/** @type {((v: Vector3) => Vector3)[]} */
	const permutations = [
		({ x, y, z }) => Vector3({ x:  x, y:  y, z:  z }),
		({ x, y, z }) => Vector3({ x:  x, y:  z, z:  y }),
		({ x, y, z }) => Vector3({ x:  y, y:  x, z:  z }),
		({ x, y, z }) => Vector3({ x:  y, y:  z, z:  x }),
		({ x, y, z }) => Vector3({ x:  z, y:  x, z:  y }),
		({ x, y, z }) => Vector3({ x:  z, y:  y, z:  x }),
	];

	/** @type {((v: Vector3) => Vector3)[]} */
	const negations = [
		({ x, y, z }) => Vector3({ x:  x, y:  y, z:  z }),
		({ x, y, z }) => Vector3({ x:  x, y:  y, z: -z }),
		({ x, y, z }) => Vector3({ x:  x, y: -y, z:  z }),
		({ x, y, z }) => Vector3({ x:  x, y: -y, z: -z }),
		({ x, y, z }) => Vector3({ x: -x, y:  y, z:  z }),
		({ x, y, z }) => Vector3({ x: -x, y:  y, z: -z }),
		({ x, y, z }) => Vector3({ x: -x, y: -y, z:  z }),
		({ x, y, z }) => Vector3({ x: -x, y: -y, z: -z }),
	];

	return xprod(permutations, negations)
			.map((fns) => compose(...fns));
})();


const input = parseInput();

{
	/** @type {(Scanner|null)[]} */ const scanners = [
		{ position: intern(Vector3({ x: 0, y: 0, z: 0 })), beacons: input[0] },
		...repeat(null, input.length - 1)
	];

	/** @type {[number, number][]} */ const noOverlap = [];

	while (scanners.some(equals(null))) {
		const [known, unknown] = partition(([/**/, scanner]) => scanner !== null, [...scanners.entries()]);

		for (const [[i, knownScanner], [j]] of permutations(known, unknown)) {
			if (noOverlap.some(p => p[0] === i && p[1] === j)) { continue; }

			const relativePosition = findRelativePosition(knownScanner, input[j]);
			if (relativePosition !== null) {
				console.log(`Found relative position of scanner ${j}:\t${Vector3.toString(relativePosition.position)}`);
				scanners[j] = relativePosition;
				break;
			} else {
				noOverlap.push([i, j]);
			}
		}
	}

	const allBeacons = Object.values(scanners)
			.flatMap(({ position, beacons }) => beacons
					.map(beacon => intern(Vector3.add(beacon, position))));
	console.log('Part 1', new Set(allBeacons).size);

	const positions = Object.values(scanners).map(prop('position'));
	console.log('Part 2', Math.max(...map(apply(Vector3.manhattanDistance), combinations(positions))));
}

/**
 * @returns {Vector3[][]}
 */
function parseInput() {
	const scanners = readFromStdin().split('\n\n');

	return scanners.map(data => data
		.split('\n')
		.slice(1)
		.map(Vector3.parse)
		.map(intern));
}

/**
 * @param {Scanner} base
 * @param {Vector3[]} beacons
 * @returns {Scanner | null}
 */
function findRelativePosition(base, beacons) {
	const directionsEqual = ([{ direction: dir1 }, { direction: dir2 }]) => dir1 === dir2;

	const f = createFingerprints(base.beacons);
	for (const transformedBeacons of possibleScannerOrientations(beacons)) {
		const pf = createFingerprints(transformedBeacons);

		const overlaps = [...filter(directionsEqual, permutations(f, pf))];
		if (overlaps.length < 66) { continue; } // 12 points can have 66 connections between them

		const bases = new Set(overlaps.map(([{ points: [p1] }, { points: [p2] }]) => intern(Vector3.subtract(p1, p2))));
		if (bases.size === 1) {
			return {
				position: intern(Vector3.add(setUtils.first(bases), base.position)),
				beacons: transformedBeacons,
			};
		}
	}

	return null;
}

/**
 * @param {Vector3[]} beacons
 * @returns {Fingerprint[]}
 */
function createFingerprints(beacons) {
	return [...permutations(beacons, beacons)]
		.filter(([x, y]) => x !== y)
		.map(points => ({
			points,
			direction: intern(Vector3.subtract(points[1], points[0])),
		}));
}

/**
 * @param {Vector3[]} beacons
 * @returns {Vector3[][]}
 */
function possibleScannerOrientations(beacons) {
	return transformationFunctions.map(fn => beacons.map(b => fn(b)));
}
