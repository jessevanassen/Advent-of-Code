/**
 * @typedef {boolean[][]} Image
 */
import { range, xprod } from 'ramda';
import { nth, readBlocksFromStdin } from '../lib/index.js';


const { algorithm, image } = parseInput();


console.log('Part 1', litPixels(nth(2,  evolutions(image))));
console.log('Part 2', litPixels(nth(50, evolutions(image))));


/**
 * @param {Image} image
 * @returns {Generator<Image>}
 */
function* evolutions(image) {
	yield image;

	for (let i = 1; /**/; i++) {
		const def = algorithm[0] && i % 2 === 0;
		image = step(image, def);
		yield image;
	}
}

/**
 *
 * @param {Image} image
 * @param {boolean} default_
 * @returns {Image}
 */
function step(image, default_) {
	return range(-1, image.length + 1)
			.map(y => range(-1, image[0].length + 1)
					.map(x => algorithm[byteAt(x, y, image, default_)]));
}

/**
 * @param {number} x
 * @param {number} y
 * @param {Image} image
 * @param {boolean} default_
 * @returns {number}
 */
function byteAt(x, y, image, default_) {
	return xprod([y-1, y, y+1], [x-1, x, x+1])
			.map(([y, x]) => image[y]?.[x] ?? default_)
			.reduce((acc, x) => acc * 2 + Number(x), 0);
}

/**
 * @param {Image} image
 * @returns {number}
 */
function litPixels(image) {
	return image.flat().filter(Boolean).length;
}

/**
 * @returns {{ algorithm: boolean[], image: Image }}
 */
function parseInput() {
	const toBoolean = c => c[0] === '#';

	const lines = [...readBlocksFromStdin()];

	const algorithm = [...lines[0]].map(toBoolean);
	const image = lines.slice(2)
			.map(line => [...line].map(toBoolean));

	return { algorithm, image };
}

/**
 * @param {Image} image
 * @returns {string}
 */
function toString(image) {
	return image
			.map(line => line
					.map(b => b ? '#' : '.')
					.join(''))
			.join('\n');
}
