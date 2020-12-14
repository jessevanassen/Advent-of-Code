import { log } from '../../lib';
import { sum } from '../../lib/fp/generators';
import { readLinesFromStdin } from '../../lib/fs';

enum CommandType { setMask, setValue };

interface SetMask {
	type: CommandType.setMask;
	value: string;
}

interface SetValue {
	type: CommandType.setValue;
	address: number;
	value: number;
}

type Command = SetMask | SetValue;

interface State {
	mask: (input: number) => number;
	memory: Record<number, number>;
}

const InitialState = (): State => ({ mask: x => x, memory: { } });

const input: Command[] = [...parseInput()];


log('Part 1:')(sum(Object.values(runPart1())));
log('Part 2:')(sum(Object.values(runPart2())));


function runPart1() {
	return input.reduce((acc, command) => {
		if (command.type === CommandType.setMask) {
			return {
				...acc,
				mask: Mask(command.value),
			};
		}
		if (command.type === CommandType.setValue) {
			return {
				...acc,
				memory: {
					...acc.memory,
					[command.address]: acc.mask(command.value),
				}
			};
		}
		return acc;
	}, InitialState()).memory;
}

function runPart2() {
	return input.reduce((acc, command) => {
		if (command.type === CommandType.setMask) {
			return { ...acc, mask: command.value };
		}
		if (command.type === CommandType.setValue) {
			let memory = { ...acc.memory };
			for (const address of combinations(acc.mask, command.address)) {
				memory[address] = command.value;
			}
			return { ...acc, memory };
		}
		return acc;
	}, { memory: {} as Record<number, number>, mask: '0'.repeat(36) }).memory;
}

function combinations(mask: string, address: number) {
	const addressBits = address.toString(2).padStart(36, '0');
	function* _sum(base: string, index: number): Generator<number> {
		if (index >= mask.length) {
			yield Number.parseInt(base, 2);
			return;
		}

		if (mask.charAt(index) === 'X') {
			yield* _sum(base + '0', index + 1)
			yield* _sum(base + '1', index + 1);
		} else if (mask.charAt(index) === '1') {
			yield* _sum(base + '1', index + 1);
		}else {
			yield* _sum(base + addressBits.charAt(index), index + 1);
		}
	}

	return _sum('', 0);
}

function* parseInput(): IterableIterator<Command> {
	for (const line of readLinesFromStdin()) {
		if (line.startsWith('mask = ')) {
			yield { type: CommandType.setMask, value: line.substring('mask = '.length) };
			continue;
		}

		const match = line.match(/^mem\[(\d+)\] = (\d+)$/)!;
		yield { type: CommandType.setValue, address: +match[1], value: +match[2] };
	}
}

function Mask(bitmask: string): (input: number) => number {
	const reverseMask = bitmask.split('').reverse();

	return (input: number) => {
		let bits = input.toString(2).padStart(36, '0').split('').reverse();
		for (let i = 0; i < bits.length; i++) {
			bits[i] = reverseMask[i] !== 'X' ? reverseMask[i] : bits[i];
		}
		return Number.parseInt(bits.reverse().join(''), 2);
	};
}
