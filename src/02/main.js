import { readBlocksFromStdin } from '../lib/index.js';

const instructions = [...readBlocksFromStdin()].map(parseInstruction);

const solve = moveFn => {
	const { position: { x, y } } = instructions.reduce(moveFn, Submarine());
	return x * y;
};

console.log('Part 1:', solve(moveAbsolute));
console.log('Part 2:', solve(moveWithAim));


function parseInstruction(instruction) {
	const [direction, amount] = instruction.split(' ', 2);
	return [direction, Number(amount)];
}

function moveAbsolute({ position, aim }, [direction, amount]) {
	const DIRECTIONS = {
		forward: Vector2({ x: 1 }),
		down: Vector2({ y: 1 }),
		up: Vector2({ y: -1 }),
	};

	return {
		aim,
		position: add(position, multiply(DIRECTIONS[direction], amount)),
	}
}

function moveWithAim({ position, aim }, [direction, amount]) {
	if (direction === 'forward') {
		return Submarine({
			position: add(position, Vector2({ x: amount, y: amount * aim })),
			aim
		});
	}

	return Submarine({
		position,
		aim: aim + (direction === 'down' ? 1 : -1) * amount,
	});
}

function Vector2({ x = 0, y = 0 } = {}) {
	return { x, y };
}

function Submarine({ position = Vector2(), aim = 0 } = {}) {
	return { position, aim };
}

function add({ x: x1, y: y1 }, { x: x2, y: y2 }) {
	return Vector2({ x: x1 + x2, y: y1 + y2 });
}

function multiply({ x, y }, n) {
	return Vector2({ x: x * n, y: y * n });
}
