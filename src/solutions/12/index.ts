import { log } from "../../lib";
import { pipe } from "../../lib/fp";
import { collectToArray, map, reduce } from "../../lib/fp/generators";
import { readLinesFromStdin } from "../../lib/fs";

type Vector = [number, number];

type Direction =  'N' | 'S' | 'E' | 'W';
type Action = Direction | 'L' | 'R' | 'F';

const directions: Record<Direction, Vector> = {
	'N': [ 0,  1],
	'S': [ 0, -1],
	'E': [ 1,  0],
	'W': [-1,  0],
};

interface Command {
	action: Action;
	value: number;
}

interface FerryState {
	position: Vector;
	direction: Vector;
}

const commands: Command[] = pipe(map(parseCommand), collectToArray)(readLinesFromStdin());

pipe(
	reduce(step(moveShip), { position: [0, 0], direction: directions['E'] }),
	state => state.position,
	manhattanDistance([0, 0]),
	log('Part 1:'),
)(commands);

pipe(
	reduce(step(moveWaypoint), { position: [0, 0], direction: [10, 1] }),
	state => state.position,
	manhattanDistance([0, 0]),
	log('Part 2:'),
)(commands);

function step(translateFn: (state: FerryState, direction: Direction, value: number) => FerryState) {
	return function _step(state: FerryState, command: Command): FerryState {
		if (isDirection(command.action)) {
			return translateFn(state, command.action, command.value);
		}

		if (command.action === 'R') {
			let direction = state.direction;
			for (let i = 0; i < command.value / 90; i++) {
				direction = rotateCW(direction);
			}
			return {
				position: state.position,
				direction
			};
		}

		if (command.action === 'L') {
			return _step(state, { action: 'R', value: (720 - command.value) % 360 });
		}

		return {
			position: add(state.position, multiplyScalar(state.direction, command.value)),
			direction: state.direction,
		};
	};
}

function moveShip(state: FerryState, direction: Direction, value: number): FerryState {
	return {
		position: add(state.position, multiplyScalar(directions[direction], value)),
		direction: state.direction,
	}
}

function moveWaypoint(state: FerryState, direction: Direction, value: number): FerryState {
	return {
		position: state.position,
		direction: add(state.direction, multiplyScalar(directions[direction], value)),
	}
}

function manhattanDistance(x: Vector) {
	return (y: Vector) => Math.abs(x[1] - x[0]) + Math.abs(y[1] - y[0]);
}

function parseCommand(command: string): Command {
	const action = command.charAt(0) as Action;
	const value = Number.parseInt(command.substring(1));
	return { action, value };
}

function add(v0: Vector, v1: Vector): Vector {
	return [
		v0[0] + v1[0],
		v0[1] + v1[1]];
}

function multiplyScalar(vector: Vector, scalar: number): Vector {
	return [vector[0] * scalar, vector[1] * scalar];
}

function rotateCW(vector: Vector): Vector {
	return [vector[1], -vector[0]];
}

function isDirection(action: Action): action is Direction {
	return ['N', 'S', 'E', 'W'].includes(action);
}
