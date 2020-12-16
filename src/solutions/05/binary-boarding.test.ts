import assert from 'assert';
import { row, column, seatId } from './binary-boarding';

assert.strictEqual(44, row('FBFBBFFRLR'));
assert.strictEqual(5, column('FBFBBFFRLR'));
assert.strictEqual(357, seatId('FBFBBFFRLR'));
