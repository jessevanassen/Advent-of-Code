export function seatId(input) {
	return row(input) * 8 + column(input);
}

export function row(input) {
	return toNumber(input.substring(0, 7));
}

export function column(input) {
	return toNumber(input.substring(7, 10));
}

function toNumber(input) {
	return [...input]
		.reduce((acc, x) => (acc << 1) | bit(x), 0);
}

function bit(input) {
	return input === 'B' || input === 'R' ? 1 : 0;
}