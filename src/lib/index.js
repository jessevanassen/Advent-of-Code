export * from './collections/index.js';
export * from './fp.js';
export * from './fs.js';
export * from './vector2.js';

/**
 * @param  {number[]} items
 * @returns {[number, number]}
 */
export function minmax(items) {
	return [Math.min(...items), Math.max(...items)];
}
