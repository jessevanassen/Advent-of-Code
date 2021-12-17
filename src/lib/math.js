/**
 * @param  {number[]} items
 * @returns {[number, number]}
 */
export function minmax(items) {
	return [Math.min(...items), Math.max(...items)];
}

/**
 * @param {number} n
 * @returns {number}
 */
export function sumRange(n) {
	return (n * (n + 1)) / 2;
}
