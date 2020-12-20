import { not, pipe } from '../../lib/fp';
import { collectToArray, filter, forEach, map } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

type Operation = 'acc' | 'jmp' | 'nop';

interface Instruction {
	operation: Operation;
	argument: number;
}

interface ProgramState {
	instructionPointer: number;
	accumulator: number;
}

type Program = Instruction[];

const InitialState = (): ProgramState => ({ instructionPointer: 0, accumulator: 0 });

const program: Program = pipe(
	map(parseInstruction),
	collectToArray,
)(readBlocksFromStdin());

console.log('Part 1:', findAccumulatorValueBeforeInfiniteLoop(program));

pipe(
	createMutations,
	filter(not(isInfinite)),
	map(run),
	forEach(log('Part 2:')),
)(program);


function findAccumulatorValueBeforeInfiniteLoop(program: Program): number {
	const seen = new Array(program.length).fill(false);

	let state = InitialState();
	while (state.instructionPointer < program.length) {
		seen[state.instructionPointer] = true;
		const newState = evaluate(program, state);

		if (seen[newState.instructionPointer]) {
			return newState.accumulator;
		}

		state = newState;
	}

	return -1;
}

function* createMutations(program: Program): IterableIterator<Program> {
	for (let i = 0; i < program.length; i++) {
		const { operation, argument } = program[i];
		switch (operation) {
		case 'jmp':
			yield replace(program, { operation: 'nop', argument }, i);
			break;
		case 'nop':
			yield replace(program, { operation: 'nop', argument }, i);
			break;
		}
	}
}

function isInfinite(program: Program): boolean {
	const seen = new Array(program.length).fill(false);
	let state = InitialState();
	while (state.instructionPointer < program.length) {
		seen[state.instructionPointer] = true;
		state = evaluate(program, state);

		if (seen[state.instructionPointer]) {
			return true;
		}
	}

	return false;
}

function run(program: Program): number {
	let state = InitialState();
	while (state.instructionPointer < program.length) {
		state = evaluate(program, state);
	}
	return state.accumulator;
}

function evaluate(program: Program, { instructionPointer, accumulator }: Readonly<ProgramState>): ProgramState {
	const { operation, argument } = program[instructionPointer];

	switch (operation) {
	case 'jmp':
		return {
			instructionPointer: instructionPointer + argument,
			accumulator,
		};
	case 'acc':
		return {
			instructionPointer: instructionPointer + 1,
			accumulator: accumulator + argument,
		};
	default:
		return {
			instructionPointer: instructionPointer + 1,
			accumulator,
		};
	}
}

function parseInstruction(line: string): Instruction {
	const match = line.match(/^(acc|jmp|nop) ([+-]\d+)$/);
	if (!match) {
		throw new Error(`Unknown instruction ${line}`);
	}

	return {
		operation: match[1] as Operation,
		argument: Number.parseInt(match[2]),
	};
}

function replace<T>(input: T[], newValue: T, index: number): T[] {
	return [...input.slice(0, index), newValue, ...input.slice(index + 1)];
}

function log(prefix?: string) {
	return (arg: unknown) => prefix ? console.log(prefix, arg) : arg;
}
