import { readFileSync } from 'fs';

export function* readLinesFromStdin(): IterableIterator<string> {
	const input = readFileSync(0, { encoding: 'utf-8', flag: 'r' });
	for (const line of input.trim().split('\n')) {
		yield line.trim();
	}
}

export function readFromStdin(): string {
	return [...readLinesFromStdin()].join('\n');
}