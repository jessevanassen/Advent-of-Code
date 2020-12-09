export function removeDuplicates<T>(input: Iterable<T>): T[] {
	return [...new Set([...input])];
}

export function* combinations2<T>(input: T[]): IterableIterator<T[]> {
	for (let i = 0; i < input.length - 1; i++) {
		for (let j = i + 1; j < input.length; j++) {
			yield [input[i], input[j]];
		}
	}
}
