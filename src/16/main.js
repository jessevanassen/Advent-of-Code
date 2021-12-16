/**
 * @typedef {4} ValueId
 * @typedef {0|1|2|3|5|6|7} OperationId
 * @typedef {ValueId | OperationId} Id
 *
 * @typedef {{ version: number; id: ValueId; value: number }} ValuePacket
 * @typedef {{ version: number; id: OperationId, subPackets: Packet[] }} OperationPacket
 * @typedef {ValuePacket|OperationPacket} Packet
 *
 * @typedef {{ matched: true, payload: Packet, remaining: boolean[] }} ParseSucceededResult
 * @typedef {{ matched: false, remaining: boolean[] }} ParseFailedResult
 * @typedef {ParseSucceededResult | ParseFailedResult} ParseResult
 *
 * @typedef {(bits: boolean[]) => ParseResult} Parser
 */

import { add, max, min, multiply } from 'ramda';
import { readFromStdin } from '../lib/index.js';

const input = parseInput();
const parseResult = parsePacket(input);


if (parseResult.matched === false) { throw new Error('Parse failed'); }

console.log('Part 1:', sumVersions(parseResult.payload));
console.log('Part 2:', interpret(parseResult.payload));


/** @type {Parser} */
function parsePacket(bits) {
	if (bits.length > 0) {
		return OrElseParser([parseLiteralPacket, parseOperationPacket])(bits);
	}

	return { matched: false, remaining: bits };
}

/**
 * @param {Parser[]} parsers
 * @returns {Parser}
 */
function OrElseParser(parsers) {
	return function(bits) {
		for (const parser of parsers) {
			const result = parser(bits);
			if (result.matched) {
				return result;
			}
		}
		return { matched: false, remaining: bits };
	}
}

/** @type {Parser} */
function parseLiteralPacket(bits) {
	const { id, version } = parseHeader(bits);

	if (isOperationId(id)) {
		return { matched: false, remaining: bits };
	}

	let remaining = bits.slice(6);
	let value = 0;
	while (true) {
		const read = fromBits(remaining.slice(0, 5));
		remaining = remaining.slice(5);
		value = value * 16 + (read & 0xF);

		if (!(read & 0b10000)) { break; }
	}

	return { matched: true, payload: { version, id, value }, remaining };
}

/** @type {Parser} */
function parseOperationPacket(bits) {
	const { id, version } = parseHeader(bits);

	if (!isOperationId(id)) {
		return { matched: false, remaining: bits };
	}

	const { payload: subPackets, remaining } = parseParameters(bits.slice(6));
	return { matched: true, payload: { version, id, subPackets }, remaining };
}

/**
 * @param {boolean[]} bits
 * @returns {{ payload: Packet[], remaining: boolean[] }}
 */
function parseParameters(bits) {
	let remaining = bits.slice(1);
	const payload = [];

	const getPacket = () => {
		const result = /** @type ParseSucceededResult */ (parsePacket(remaining));
		remaining = result.remaining;
		return result.payload;
	}

	if (bits[0] === false) {
		const bitCount = fromBits(remaining.slice(0, 15));
		remaining = remaining.slice(15);

		while (bits.length - 16 - remaining.length < bitCount) {
			payload.push(getPacket());
		}
	} else {
		const packetCount = fromBits(remaining.slice(0, 11));
		remaining = remaining.slice(11);

		for (let i = 0; i < packetCount; i++) {
			payload.push(getPacket());
		}
	}

	return { payload, remaining };
}

/**
 * @param {Id} id
 * @returns {id is OperationId}
 */
function isOperationId(id) {
	return id !== 4;
}

/**
 * @param {boolean[]} bits
 * @returns {{ version: number; id: Id }}
 */
function parseHeader(bits) {
	return {
		version: fromBits(bits.slice(0, 3)),
		id: /** @type {Id} */ (fromBits(bits.slice(3, 6))),
	};
}

/**
 * @param {Packet} packet
 * @returns {number}
 */
function sumVersions(packet) {
	if (packet.id === 4) { return packet.version; }

	return packet.subPackets
			.map(sumVersions)
			.reduce(add, packet.version);
}

/**
 * @param {Packet} packet
 * @returns {number}
 */
function interpret(packet) {
	if (packet.id === 4) { return packet.value; }

	const subValues = packet.subPackets.map(interpret);

	switch (packet.id) {
		case 0: return subValues.reduce(add);
		case 1: return subValues.reduce(multiply);
		case 2: return subValues.reduce(min);
		case 3: return subValues.reduce(max);
		case 5: return subValues[0] >   subValues[1] ? 1 : 0;
		case 6: return subValues[0] <   subValues[1] ? 1 : 0;
		case 7: return subValues[0] === subValues[1] ? 1 : 0;
	}
}

/**
 * @returns {boolean[]}
 */
function parseInput() {
	return [...readFromStdin()]
			.map(c => Number.parseInt(c, 16))
			.flatMap(toBits);
}

/**
 * @param {number} byte
 * @returns {[boolean, boolean, boolean, boolean]}
 */
function toBits(byte) {
	return [
		!!((byte >> 3) & 1),
		!!((byte >> 2) & 1),
		!!((byte >> 1) & 1),
		!!((byte >> 0) & 1),
	];
}

/**
 * @param {boolean[]} bits
 * @returns {number}
 */
function fromBits(bits) {
	return bits.reduce((acc, b) => acc * 2 + Number(b), 0);
}
