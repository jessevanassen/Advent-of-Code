/**
 * @typedef {{ x: number, y: number }} Vector2
 */

export function Vector2({ x = 0, y = 0 } = {}) {
	return { x, y };
}

/**
 * @param {string} string
 */
Vector2.parse = function(string) {
	const [/**/, x, y] = string.match(/\(?(\d+(?:\.\d+)*),\w*(\d+(?:\.\d+)*)\)?/);
	return Vector2({ x: parseFloat(x), y: parseFloat(y) });
};

/**
 * @param {Vector2} param0
 */
Vector2.toString = function({ x, y }) {
	return `(${x}, ${y})`;
};
