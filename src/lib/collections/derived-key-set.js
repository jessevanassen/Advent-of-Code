import { DerivedKeyMap } from "./derived-key-map.js";

/**
 * @template T
 * @typedef {{
 *   add: (v: T) => void,
 *   delete: (v: T) => boolean,
 *   has: (v: T) => boolean,
 *   size: number,
 *   values: () => Iterable<T>,
 *   get: (v: T) => T,
 * }} DerivedKeySet
 */

/**
 * @template T
 * @param {(x: T) => unknown} mapper
 * @returns {DerivedKeySet<T>}
 */
export function DerivedKeySet(mapper) {
	/** @type {DerivedKeyMap<T, T>} */
	const map = DerivedKeyMap(mapper);

	return {
		add: v => { map.set(v, v); },
		delete: k => map.delete(k),
		has: k => map.has(k),
		get size() { return map.size; },
		values: () => map.values(),

		get: v => map.get(v),
	};
}
