import { log } from '../../lib';
import { pipe } from '../../lib/fp';
import { collectToArray, map, sum } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

type Operator = '+' | '*';
type Token = Operator | '(' | ')' | number;
type BinaryOperation = { operator: Operator; left: Expression; right: Expression };
type Expression = BinaryOperation | number;

const tokens = pipe(
	map(tokenize),
	collectToArray,
)(readBlocksFromStdin());

pipe(
	map(parseExpressionSamePrecedence),
	map(evaluateExpression),
	sum,
	log('Part 1:'),
)(tokens);

pipe(
	map(parseExpressionDifferentPrecedence),
	map(evaluateExpression),
	sum,
	log('Part 2:'),
)(tokens);

function tokenize(line: string): Token[] {
	return line.match(/[+*()]|\d+/g)?.map(parseToken) ?? [];

	function parseToken(token: string): Token {
		if (token === '+' || token === '*' || token === '(' || token === ')') {
			return token;
		}
		return Number.parseInt(token);
	}
}

function parseExpressionSamePrecedence(tokens: Token[]): Expression {
	/**
	 * operation = group ( ('+' | '*') group )*
	 * group = number | '(' operation ')'
	 */
	let index = 0;

	function parseGroup(): Expression {
		if (typeof tokens[index] === 'number') {
			return tokens[index++] as number;
		}
		index++;
		const next = parseOperation();
		index++;
		return next;
	}

	function parseOperation(): Expression {
		let operation = parseGroup();

		while (tokens[index] === '*' || tokens[index] === '+') {
			const operator = tokens[index++] as Operator;
			const right = parseGroup();
			operation = { left: operation, operator, right };
		}

		return operation;
	}

	return parseOperation();
}

function parseExpressionDifferentPrecedence(tokens: Token[]): Expression {
	/**
	 * multiplication = addition ( '*' addition )*
	 * addition = group ( '+' group )*
	 * group = number | '(' multiplication ')'
	 */
	let index = 0;

	function parseGroup(): Expression {
		if (typeof tokens[index] === 'number') {
			return tokens[index++] as number;
		}
		index++;
		const next = parseMultiplication();
		index++;
		return next;
	}

	function parseAddition(): Expression {
		let operation = parseGroup();

		while (tokens[index] === '+') {
			const operator = tokens[index++] as Operator;
			const right = parseGroup();
			operation = { left: operation, operator, right };
		}

		return operation;
	}

	function parseMultiplication(): Expression {
		let operation = parseAddition();

		while (tokens[index] === '*') {
			const operator = tokens[index++] as Operator;
			const right = parseAddition();
			operation = { left: operation, operator, right };
		}

		return operation;
	}

	return parseMultiplication();
}

function evaluateExpression(expression: Expression): number {
	if (typeof expression === 'number') return expression;

	const { left, right, operator } = expression;
	if (operator === '*') return evaluateExpression(left) * evaluateExpression(right);
	if (operator === '+') return evaluateExpression(left) + evaluateExpression(right);

	throw new Error(`Unreachable: ${JSON.stringify(expression, undefined, '\t')}`);
}
