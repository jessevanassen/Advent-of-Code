export function removeDuplicates<T>(input: Iterable<T>): T[] {
	return [...new Set([...input])];
}
