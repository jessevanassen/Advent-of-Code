/**
 * @typedef {Record<string, number>} Occurrences
 */

import { aperture, complement, countBy, equals, identity, join, lensProp, map, over, pickBy } from 'ramda';
import { generate, readFromStdin, take, minmax } from '../lib/index.js';

const { template, rules } = parseInput();

const results = [...take(41, generate(evolve, countPairs(template)))];

console.log('Part 1', answer(results[10]));
console.log('Part 2', answer(results[40]));


/**
 * @param {Occurrences} pairCounts
 * @returns {number}
 */
function answer(pairCounts) {
	/** @type {Occurrences} */ let letterCount = {};

	for (const [pair, count] of Object.entries(pairCounts)) {
		for (const letter of pair) {
			letterCount = addToProp(letter, count, letterCount);
		}
	}
	letterCount[template.at(0)]++;
	letterCount[template.at(-1)]++;

	const [min, max] = minmax(Object.values(letterCount));
	return (max - min) / 2;
}

/**
 * @param {Occurrences} counts
 * @returns {Occurrences}
 */
function evolve(counts) {
	for (const [pair, count] of Object.entries(counts)) {
		const t = rules[pair];

		counts = addToProp(pair[0] + t, count, counts);
		counts = addToProp(t + pair[1], count, counts);
		counts = addToProp(pair,       -count, counts)
	}

	return pickBy(complement(equals(0)), counts);
}

/**
 * @param {string} template
 * @returns {Occurrences}
 */
function countPairs(template) {
	return countBy(identity, letterPairs(template));
}

/**
 * @param {string} string
 * @returns {string[]}
 */
function letterPairs(string) {
	return map(join(''), aperture(2, [...string]));
}

/**
 * @param {string} key
 * @param {number} value
 * @param {Occurrences} obj
 * @returns {Occurrences}
 */
function addToProp(key, value, obj) {
	return over(lensProp(key), v => (v ?? 0) + value, obj);
}

function parseInput() {
	const [template, rules] = readFromStdin().split('\n\n', 2);
	return {
		template,
		rules: Object.fromEntries(rules.split('\n').map(rule => rule.split(' -> ', 2)))
	}
}
