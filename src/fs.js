import { readFileSync } from 'fs';

/**
 * @param {string} [separator]
 * @returns {IterableIterator<string>}
 */
export function* readBlocksFromStdin(separator = '\n') {
	const input = readFileSync(0, { encoding: 'utf-8', flag: 'r' });
	for (const line of input.trim().split(separator)) {
		yield line.trim();
	}
}

/**
 * @returns {string}
 */
export function readFromStdin() {
	return [...readBlocksFromStdin()].join('\n');
}
