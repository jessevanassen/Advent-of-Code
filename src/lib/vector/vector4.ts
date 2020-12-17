export type Vector4 = [number, number, number, number];

export function min(first: Vector4, second: Vector4): Vector4 {
	return [
		Math.min(first[0], second[0]),
		Math.min(first[1], second[1]),
		Math.min(first[2], second[2]),
		Math.min(first[3], second[3]),
	];
}

export function max(first: Vector4, second: Vector4): Vector4 {
	return [
		Math.max(first[0], second[0]),
		Math.max(first[1], second[1]),
		Math.max(first[2], second[2]),
		Math.max(first[3], second[3]),
	];
}

export function addScalar(scalar: number) {
	return (x: Vector4): Vector4 => [x[0] + scalar, x[1] + scalar, x[2] + scalar, x[3] + scalar];
}

export function toString(vector4: Vector4): string {
	return `[${vector4[0]},${vector4[1]},${vector4[2]},${vector4[3]}]`;
}

export function fromString(Vector4: string): Vector4 {
	return Vector4.slice(1, -1)
		.split(',', 4)
		.map(x => Number.parseFloat(x)) as Vector4;
}

export function equals(x: Vector4, y: Vector4): boolean {
	return x[0] === y[0] && x[1] === y[1] && x[2] === y[2] && x[3] === y[3];
}
