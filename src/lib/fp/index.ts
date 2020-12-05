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

export function pipe<T0, T1, T2>(...fns: [(x: T0) => T1, (x: T1) => T2]): (x: T0) => T2
export function pipe<T0, T1, T2, T3>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3]): (x: T0) => T3
export function pipe<T0, T1, T2, T3, T4, T5>(...fns: [(x: T0) => T1, (x: T1) => T2, (x: T2) => T3, (x: T3) => T4, (x: T4) => T5]): (x: T0) => T5
export function pipe(...fns: ((x: any) => any)[]) {
	return (x: any) => fns.reduce((acc, fn) => fn(acc), x);
}
