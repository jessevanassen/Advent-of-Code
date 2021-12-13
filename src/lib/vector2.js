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
	const [/**/, x, y] = string.match(/\(?(\d+(?:\.\d+)*),\s*(\d+(?:\.\d+)*)\)?/);
	return Vector2({ x: parseFloat(x), y: parseFloat(y) });
};

/**
 * @param {Vector2} v1
 * @param {Vector2} v2
 * @returns {Vector2}
 */
Vector2.add = function(v1, v2) {
	return {
		x: v1.x + v2.x,
		y: v1.y + v2.y,
	}
}

/**
 * @param {Vector2} v1
 * @param {Vector2} v2
 * @returns {Vector2}
 */
Vector2.subtract = function(v1, v2) {
	return {
		x: v1.x - v2.x,
		y: v1.y - v2.y,
	}
}

/**
 * @param {Vector2} v1
 * @param {Vector2} v2
 * @returns {Vector2}
 */
Vector2.multiply = function(v1, v2) {
	return {
		x: v1.x * v2.x,
		y: v1.y * v2.y,
	}
}

/**
 * @param {Vector2} param0
 */
 Vector2.toString = function({ x, y }) {
	return `(${x}, ${y})`;
};
