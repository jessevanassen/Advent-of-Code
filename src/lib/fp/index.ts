export function eq(value: unknown) {
	return (x: unknown): boolean => x === value;
}

type Falsy = false | 0 | '' | null | undefined;

export const truthy = <T>(x: T | Falsy): x is T => !!x;

export function count(iterable: Iterable<unknown>): number {
	return Array.isArray(iterable) ?
		iterable.length :
		[...iterable].length;
}

export function inspect<T>(x: T): T {
	console.debug(x);
	return x;
}

export function replace(pattern: RegExp, replacement: string) {
	return (input: string): string => input.replace(pattern, replacement);
}

export function not<T>(fn: (arg: T) => boolean): (arg: T) => boolean {
	return (arg: T) => !fn(arg);
}

type Comparator<T> = (x: T, y: T) => number;

export function sort<T>(comparator: Comparator<T>) {
	return function(input: Iterable<T>): T[] {
		return [...input].sort(comparator);
	};
}

export const numberComparator: Comparator<number> = (x, y) =>
	x < y ? -1 :
		x > y ? 1 :
			0;

export function reverseComparator<T>(comparator: Comparator<T>): Comparator<T> {
	return function(x, y) {
		return comparator(x, y) * -1;
	};
}

export function mapObject<T, U>(fn: (x: T) => U) {
	return function<K extends string>(obj: Record<K, T>): Record<K, U> {
		const result = {} as Record<string, U>;
		for (const [key, value] of Object.entries<T>(obj)) {
			result[key] = fn(value);
		}
		return result as Record<K, U>;
	};
}

export function pipe<T0, T1>(...fns: [(x: T0) => T1]): (x: T0) => T1
export function pipe<T0, T1, T2>(...fns: [(x: T0) => T1, (x: T1) => T2]): (x: T0) => T2
export function pipe<T0, T1, T2, T3>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3]): (x: T0) => T3
export function pipe<T0, T1, T2, T3, T4>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4]): (x: T0) => T4
export function pipe<T0, T1, T2, T3, T4, T5>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5]): (x: T0) => T5
export function pipe<T0, T1, T2, T3, T4, T5, T6>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5, (x: T5) => T6]): (x: T0) => T6
export function pipe<T0, T1, T2, T3, T4, T5, T6, T7>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5, (x: T5) => T6, (x: T6) => T7]): (x: T0) => T7
export function pipe<T0, T1, T2, T3, T4, T5, T6, T7, T8>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5, (x: T5) => T6, (x: T6) => T7, (x: T7) => T8]): (x: T0) => T8
export function pipe<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5, (x: T5) => T6, (x: T6) => T7, (x: T7) => T8, (x: T8) => T9]): (x: T0) => T9
export function pipe<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5, (x: T5) => T6, (x: T6) => T7, (x: T7) => T8, (x: T8) => T9, (x: T9) => T10]): (x: T0) => T10
export function pipe(...fns: ((x: unknown) => unknown)[]) {
	return (x: unknown): unknown => fns.reduce((acc, fn) => fn(acc), x);
}
