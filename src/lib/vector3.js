/**
 * @typedef {{ x: number, y: number, z: number }} Vector3
 */

export function Vector3({ x = 0, y = 0, z = 0 } = {}) {
	return { x, y, z };
}

/**
 * @param {string} string
 */
Vector3.parse = function(string) {
	const [/**/, x, y, z] = string.match(/\(?(-?\d+(?:\.\d+)*),\s*(-?\d+(?:\.\d+)*),\s*(-?\d+(?:\.\d+)*)\)?/);
	return Vector3({ x: parseFloat(x), y: parseFloat(y), z: parseFloat(z) });
};

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {boolean}
 */
Vector3.equals = function(v1, v2) {
	return v1.x === v2.x && v1.y === v2.y && v1.z === v2.z;
}

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {Vector3}
 */
Vector3.add = function(v1, v2) {
	return {
		x: v1.x + v2.x,
		y: v1.y + v2.y,
		z: v1.z + v2.z,
	}
}

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {Vector3}
 */
Vector3.subtract = function(v1, v2) {
	return {
		x: v1.x - v2.x,
		y: v1.y - v2.y,
		z: v1.z - v2.z,
	}
}

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {Vector3}
 */
Vector3.product = function(v1, v2) {
	return {
		x: v1.x * v2.x,
		y: v1.y * v2.y,
		z: v1.z * v2.z,
	}
}

/**
 * @param {Vector3} param0
 */
Vector3.toString = function({ x, y, z }) {
	return `(${x}, ${y}, ${z})`;
};

Vector3.magnitude = function({ x, y, z }) {
	return (x**2 + y**2 + z**2) ** 0.5;
}

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {number}
 */
Vector3.distance = function(v1, v2) {
	return Vector3.magnitude(Vector3.subtract(v2, v1));
}

/**
 * @param {Vector3} v1
 * @param {Vector3} v2
 * @returns {number}
 */
Vector3.manhattanDistance = function({ x: x1, y: y1, z: z1 }, { x: x2, y: y2, z: z2 }) {
	return Math.abs(x2 - x1) + Math.abs(y2 - y1) + Math.abs(z2 - z1);
}
