/**
 * @template T
 * @param {Set<T>} s1
 * @param {Set<T>} s2
 * @returns {Set<T>}
 */
export function intersection(s1, s2) {
	const result = new Set();
	for (const item of s1) {
		if (s2.has(item)) {
			result.add(item);
		}
	}
	return result;
}

/**
 * @template T
 * @param {Set<T>} s
 * @returns {T}
 */
export function first(s) {
	return s[Symbol.iterator]().next().value;
}
