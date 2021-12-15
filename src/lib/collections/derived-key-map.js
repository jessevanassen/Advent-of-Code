/**
 * @template K, T
 * @typedef {{
 *   get: (k: K) => T,
 *   set: (k: K, v: T) => void,
 *   delete: (k: K) => boolean,
 *   has: (k: K) => boolean,
 *   size: number,
 *   values: () => Iterable<T>,
 * }} DerivedKeyMap
 */

/**
 * @template K, T
 * @param {(k: K) => unknown} mapper
 * @returns {DerivedKeyMap<K, T>}
 */
export function DerivedKeyMap(mapper) {
	const map = new Map();

	return {
		get: k => map.get(mapper(k)),
		set: (k, v) => { map.set(mapper(k), v); },
		delete: k => map.delete(mapper(k)),
		has: k => map.has(mapper(k)),
		get size() { return map.size; },
		values: () => map.values(),
	};
}
