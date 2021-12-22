import { DerivedKeySet } from './collections/derived-key-set.js';

/**
 * @template T
 * @param {(item: T) => unknown} keyFn
 * @returns {(item: T) => T}
 */
export function Interner(keyFn) {
	const interned = DerivedKeySet(keyFn);

	return function(item) {
		const fromInterned = interned.get(item);

		if (fromInterned) { return fromInterned; }

		interned.add(item);
		return item;
	}
}
