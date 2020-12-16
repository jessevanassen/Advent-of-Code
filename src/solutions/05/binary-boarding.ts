export function seatId(input: string): number {
	return row(input) * 8 + column(input);
}

export function row(input: string): number {
	return toNumber(input.substring(0, 7));
}

export function column(input: string): number {
	return toNumber(input.substring(7, 10));
}

function toNumber(input: string): number {
	return [...input]
		.reduce((acc, x) => (acc << 1) | bit(x), 0);
}

function bit(input: string): 1 | 0 {
	return input === 'B' || input === 'R' ? 1 : 0;
}
