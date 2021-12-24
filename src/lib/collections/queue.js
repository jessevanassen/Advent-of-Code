/**
 * @template T
 */
export function Queue() {
	/** @type {T[]} */
	let items = new Array(8);
	let start = 0;
	let end = 0;
	let count = 0;

	/**
	 * @param {number} i
	 * @returns {number}
	 */
	function index(i) {
		return (i + items.length) % items.length;
	}

	/**
	 * @param {number} newSize
	 */
	function resize(newSize) {
		const newArray = new Array(newSize);

		let i = 0;
		if (start < end) {
			for (let j = start; j < end; j++) { newArray[i++] = items[j]; }
		} else {
			for (let j = start; j < items.length; j++) { newArray[i++] = items[j]; }
			for (let j = 0; j < end; j++) { newArray[i++] = items[j]; }
		}

		items = newArray;
		start = 0;
		end = i;
		count = end;
	}

	/**
	 * @param {T} item
	 */
	function enqueue(item) {
		if (size() >= items.length) {
			resize(items.length * 2);
		}

		items[end] = item;
		end = index(end + 1);
		count++;
	}

	/**
	 * @returns {T}
	 */
	function dequeue() {
		if (isEmpty()) { return; }

		const item = items[start];
		items[start] = undefined;
		start = index(start + 1);
		count--;

		return item;
	}

	function size() {
		return count;
	}

	/**
	 * @returns {boolean}
	 */
	function isEmpty() {
		return size() === 0;
	}

	return { enqueue, dequeue, size, isEmpty };
}
