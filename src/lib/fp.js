/**
 * @param {{ from?: number; to: number; step?: number }} param0
 */
export function* range({ from = 0, to, step = 1 }) {
	for (let i = from; i < to; i += step) {
		yield i;
	}
}
