/**
 * @param {{ from?: number; to: number; step?: number }} param0
 */
export function* range({ from = 0, to, step = 1 }) {
	for (let i = from; i < to; i += step) {
		yield i;
	}
}

/**
 * @template T
 * @param {Iterable<T>} iterable
 * @returns {T}
 */
 export function first(iterable) {
	for (const item of iterable) {
		return item;
	}
}

/**
 * @template T, U
 * @param {(t: T) => U} mapper
 * @param {Iterable<T>} iterable
 * @returns {IterableIterator<U>}
 */
export function* map(mapper, iterable) {
	for (const item of iterable) {
		yield mapper(item);
	}
}

/**
 * @template T
 * @param {(t: T) => boolean} predicate
 * @param {Iterable<T>} iterable
 * @returns {IterableIterator<T>}
 */
export function* filter(predicate, iterable) {
	for (const item of iterable) {
		if (predicate(item)) {
			yield item;
		}
	}
}

/**
 * @template T
 * @param {(t: T) => boolean} predicate
 * @param {Iterable<T>} iterable
 * @returns {T}
 */
export function find(predicate, iterable) {
	return first(filter(predicate, iterable));
}

/**
 * @template T
 * @param {(t: T) => boolean} predicate
 * @param {Iterable<T>} iterable
 * @returns {number}
 */
export function findIndex(predicate, iterable) {
	return first(filter(
		([item]) => predicate(item),
		zipWithIndex(iterable))
	)[1];
}

/**
 * @template T, U
 * @param {(x: U, y: T) => U} accumulator
 * @param {Iterable<T>} iterable
 * @param {U} initial
 * @returns {U}
 */
export function fold(accumulator, initial, iterable) {
	for (const item of iterable) {
		initial = accumulator(initial, item);
	}
	return initial;
}

/**
 * @template T, U
 * @param {(x: U, y: T) => U} accumulator
 * @param {Iterable<T>} iterable
 * @param {U} initial
 * @returns {IterableIterator<U>}
 */
export function* scan(accumulator, initial, iterable) {
	for (const item of iterable) {
		initial = accumulator(initial, item);
		yield initial;
	}
}

/**
 * @template T
 * @param {Iterable<T>} iterable
 * @returns {IterableIterator<[T, number]>}
 */
export function* zipWithIndex(iterable) {
	let i = 0;
	for (const item of iterable) {
		yield [item, i++];
	}
}

/**
 * @template T
 * @param {number} n
 * @param {Iterable<T>} iterable
 * @returns {IterableIterator<T>}
 */
export function* skip(n, iterable) {
	for (const item of iterable) {
		if (n-- > 0) {
			continue;
		}
		yield item;
	}
}

/**
 * @template T
 * @param {number} n
 * @param {Iterable<T>} iterable
 * @returns {IterableIterator<T>}
 */
export function* take(n, iterable) {
	for (const item of iterable) {
		if (n-- <= 0) { return; }
		yield item;
	}
}

/**
 * @template T
 * @param {(x: T) => T} nextFn
 * @param {T} initial
 * @returns {IterableIterator<T>}
 */
export function* generate(nextFn, initial) {
	while (true) {
		yield initial;
		initial = nextFn(initial);
	}
}

/**
 * @template T
 * @param {number} n
 * @param {Iterable<T>} iterable
 * @returns T
 */
export function nth(n, iterable) {
	return first(skip(n, iterable));
}

/**
 * @template T
 * @param {Iterable<T>} iterable
 * @returns {T[]}
 */
export function collect(iterable) {
	return [...iterable];
}

/**
 * @param {Iterable<unknown>} iterable
 */
export function count(iterable) {
	return fold(x => x + 1, 0, iterable);
}
