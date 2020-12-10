import { not } from '..';

export function* range(end: number, { start = 0, step = 1 } = {}): IterableIterator<number> {
	for (let i = start; i < end; i += step) {
		yield i;
	}
}

export function map<T, U>(fn: (x: T) => U): (iterable: Iterable<T>) => IterableIterator<U> {
	return function*(iterable: Iterable<T>) {
		for (const x of iterable) {
			yield fn(x);
		}
	};
}

export function* flatten<T>(iterable: Iterable<Iterable<T>>) {
	for (const i of iterable) {
		yield* i;
	}
}

export function flatMap<T, U>(fn: (x: T) => Iterable<U>): (iterable: Iterable<T>) => IterableIterator<U> {
	return function(iterable: Iterable<T>) {
		return flatten(map(fn)(iterable));
	}
}

export function forEach<T>(fn: (x: T) => void) {
	return (iterable: Iterable<T>) => {
		for (const x of iterable) {
			fn(x);
		}
	};
}

export function filter<T>(predicate: (x: T) => boolean): (iterable: Iterable<T>) => IterableIterator<T> {
	return function*(iterable: Iterable<T>) {
		for (const x of iterable) {
			if (predicate(x)) {
				yield x;
			}
		}
	};
}

export function reduce<T, U>(fn: (acc: U, x: T) => U, initial?: U): (iterable: Iterable<T>) => U {
	return (iterable: Iterable<T>) => {
		const iterator = iterable[Symbol.iterator]();
		let acc = initial ?? iterator.next().value;

		while (true) {
			const { done, value } = iterator.next();
			if (done)
				break;

			acc = fn(acc, value);
		}

		return acc;
	};
}

export const sum = reduce((x, y: number) => x + y, 0);

export const product = reduce((x, y: number) => x * y, 1);

export function max(iterable: Iterable<number>): number {
	return Math.max(...iterable);
}

export function collectToArray<T>(iterable: Iterable<T>): T[] {
	return [...iterable];
}

export function fromEntries<T extends string | number, U>(iterable: Iterable<[T, U]>): Record<T, U> {
	return reduce((acc, [key, value]: [T, U]) => (acc[key] = value, acc), {} as Record<T, U>)(iterable);
}

export function first<T>(iterable: Iterable<T>): T | undefined {
	return iterable[Symbol.iterator]().next().value;
}

export function any<T>(fn = (x: T) => !!x): (iterable: Iterable<T>) => boolean {
	return iterable => {
		for (const i of iterable) {
			if (fn(i)) {
				return true;
			}
		}
		return false;
	}
}

export function all<T>(fn = (x: T) => !!x): (iterable: Iterable<T>) => boolean {
	return not(any<T>(not(fn)));
}

export function split(separator = ''): (input: string) => IterableIterator<string> {
	if (separator === '') {
		return (input: string) => input[Symbol.iterator]();
	}

	return function*(input) {
		let start = 0;
		let end;
		while ((end = input.indexOf(separator, start)) !== -1) {
			yield input.substring(start, end);
			start = end + separator.length;
		}

		yield input.substring(start);
	};
}

export function take<T>(n: number) {
	return function*(iterable: Iterable<T>): IterableIterator<T> {
		const iterator = iterable[Symbol.iterator]();
		let i = 0;
		while (true) {
			const { value, done } = iterator.next();
			if (!done && i++ < n) {
				yield value;
			}
		}
	}
}

export function takeWhile<T>(predicate: (x: T) => boolean) {
	return function*(iterable: Iterable<T>): IterableIterator<T> {
		const iterator = iterable[Symbol.iterator]();
		while (true) {
			const { value, done } = iterator.next();
			if (!done && predicate(value)) {
				yield value;
			}
		}
	}
}

export function* aperture<T>(iterable: Iterable<T>): IterableIterator<[T, T]> {
	const iterator = iterable[Symbol.iterator]();
	let prev = iterator.next().value;
	while (true) {
		const { done, value } = iterator.next();
		if (done) {
			break;
		}

		yield [prev, value];
		prev = value;
	}
}

export function skip<T>(n: number) {
	return function*(iterable: Iterable<T>) {
		if (Array.isArray(iterable)) {
			for (let i = n; i < iterable.length; i++) {
				yield iterable[i]
			}
			return;
		}

		const iterator = iterable[Symbol.iterator]();
		for (let i = 0; i < n; i++) {
			iterator.next();
		}

		while (true) {
			const { value, done } = iterator.next();
			console.log(value, done);
			if (done) break;
			yield value;
		}
	}
}
