import { pipe } from "../../lib/fp";
import { collect, filter, first, forEach, map } from "../../lib/fp/generators";
import { readLinesFromStdin } from "../../lib/fs";

const expenses = pipe(
	map(Number),
	collect,
)(readLinesFromStdin());

const expenseReport: (combinations: Iterable<number[]>) => number | undefined = pipe(
	filter(x => add(...x) === 2020),
	map(x => multiply(...x)),
	first,
);

console.log(expenseReport(combinations2(expenses)));
console.log(expenseReport(combinations3(expenses)));


function* combinations2(input: number[]): IterableIterator<number[]> {
	for (let i = 0; i < input.length - 1; i++) {
		for (let j = i + 1; j < input.length; j++) {
			yield [input[i], input[j]];
		}
	}
}

function* combinations3(input: number[]): IterableIterator<number[]> {
	for (let i = 0; i < input.length - 2; i++) {
		for (let j = i + 1; j < input.length - 1; j++) {
			for (let k = j + 1; k < input.length; k++) {
				yield [input[i], input[j], input[k]];
			}
		}
	}
}

function add(...args: number[]) {
	return args.reduce((x, y) => x + y, 0);
}

function multiply(...args: number[]) {
	return args.reduce((x, y) => x * y, 1);
}
