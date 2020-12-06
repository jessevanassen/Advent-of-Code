export function eq(value: unknown) {
	return (x: unknown) => x === value;
}

export const truthy = (x: any) => !!x;

export function length(iterable: Iterable<any>): number {
	return Array.isArray(iterable) ?
		iterable.length :
		[...iterable].length;
}

export function inspect<T>(x: T): T {
	console.debug(x);
	return x;
}

export function replace(pattern: RegExp, replacement: string) {
	return (input: string) => input.replace(pattern, replacement);
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
export function pipe(...fns: ((x: any) => any)[]) {
	return (x: any) => fns.reduce((acc, fn) => fn(acc), x);
}
