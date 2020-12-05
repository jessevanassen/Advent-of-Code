export function* range(end: number, { start = 0, step = 1 } = {}): IterableIterator<number> {
	for (let i = start; i < end; i += step) {
		yield i;
	}
}

export function map<X, Y>(fn: (x: X) => Y): (iterable: Iterable<X>) => IterableIterator<Y> {
	return function*(iterable: Iterable<X>) {
		for (const x of iterable) {
			yield fn(x);
		}
	};
}

export function forEach<X>(fn: (x: X) => void) {
	return (iterable: Iterable<X>) => {
		for (const x of iterable) {
			fn(x);
		}
	};
}

export function filter<X>(predicate: (x: X) => boolean): (iterable: Iterable<X>) => IterableIterator<X> {
	return function*(iterable: Iterable<X>) {
		for (const x of iterable) {
			if (predicate(x)) {
				yield x;
			}
		}
	};
}

export function reduce<X, Y>(fn: (acc: Y, x: X) => Y, initial?: Y): (iterable: Iterable<X>) => Y {
	return (iterable: Iterable<X>) => {
		const iterator = iterable[Symbol.iterator]();
		let acc = initial || iterator.next().value;

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

export function collect<T>(iterable: Iterable<T>): T[] {
	return [...iterable];
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
