import { readFileSync } from 'fs';

export function* readBlocksFromStdin(separator = '\n'): IterableIterator<string> {
	const input = readFileSync(0, { encoding: 'utf-8', flag: 'r' });
	for (const line of input.trim().split(separator)) {
		yield line.trim();
	}
}

export function readFromStdin(): string {
	return [...readBlocksFromStdin()].join('\n');
}
