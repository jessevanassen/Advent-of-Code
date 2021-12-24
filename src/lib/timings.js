export function Timings() {
	/** @type {Map<string, { calls: number; time: number }>} */
	const timings = new Map();

	/**
	 * @template T
	 * @param {string} label
	 * @param {() => T} fn
	 * @returns {T}
	 */
	function measure(label, fn) {
		const start = performance.now();
		const result = fn();
		const end = performance.now();

		const previous = timings.get(label) ?? { calls: 0, time: 0 };
		timings.set(label, { time: previous.time + (end - start), calls: previous.calls + 1 });

		return result;
	}

	/**
	 * @returns {void}
	 */
	function report() {
		const report = Object.fromEntries([...timings.entries()]
				.map(([label, { time, calls }]) => [label, {
					time,
					calls,
					'average time per call': time / calls,
				}]));
		console.table(report);
	}

	return { measure, report };
}
