/**
 * @typedef {{ parent:  Branch; value: number }} Leaf
 * @typedef {{ parent?: Branch; left: Node; right: Node }} Branch
 * @typedef {{ parent:  Branch; left: Leaf; right: Leaf }} LeafBranch
 * @typedef { Leaf | Branch } Node
 */

import { equals, max, splitWhen, xprod } from 'ramda';
import { readBlocksFromStdin } from '../lib/index.js';

const input = parseInput();

console.log('Part 1', magnitude(input.reduce(addNodes)));
console.log('Part 2', xprod(input, input)
		.filter(([x, y]) => x !== y)
		.map(([x, y]) => addNodes(x, y))
		.map(magnitude)
		.reduce(max)
);


/**
 * @param {Node} node1
 * @param {Node} node2
 * @returns {Node}
 */
function addNodes(node1, node2) {
	const node = {
		left: node1,
		right: node2,
	};
	node1.parent = node;
	node2.parent = node;

	return reduceNode(node);
}

/**
 * @param {Node} node
 * @returns {Node}
 */
function reduceNode(node) {
	return untilStable(n => {
		let n_ = explodeNode(n);
		if (equals(n_, n)) {
			n_ = splitNode(n_);
		}
		return n_;
	}, node);
}

/**
 * @param {Node} node
 * @returns {Node}
 */
function explodeNode(node) {
	node = clone(node);

	const flattened = flatten(node);
	const toExplode = flattened.find(isExplodableBranch);

	if (!toExplode) { return node; }

	const [leftNodes, rightNodes] = splitWhen(x => x === toExplode, flattened);
	const leftSibling = leftNodes
			.filter(isLeaf)
			.at(-2);
	const rightSibling = rightNodes
			.filter(isLeaf)
			.at(1);

	if (leftSibling) { leftSibling.value += toExplode.left.value; }
	if (rightSibling) { rightSibling.value += toExplode.right.value; }

	replace(toExplode, { value: 0 });

	return node;
}

/**
 * @param {Node} node
 * @returns {Node}
 */
function splitNode(node) {
	node = fromString(toString(node));

	const toSplit = flatten(node)
			.filter(isLeaf)
			.find(({ value }) => value >= 10);

	if (!toSplit) { return node; }

	replace(toSplit, LeafBranch(
		Math.floor(toSplit.value / 2),
		Math.ceil(toSplit.value / 2),
	));

	return node;
}

/**
 * @param {number} v1
 * @param {number} v2
 * @returns {Node}
 */
function LeafBranch(v1, v2) {
	const parent = {};
	parent.left = { value: v1, parent };
	parent.right = { value: v2, parent };
	return parent;
}

/**
 * @param {Node} node
 * @returns {node is Branch}
 */
function isBranch(node) {
	return 'left' in node && 'right' in node;
}

/**
 * @param {Node} node
 * @returns {node is Leaf}
 */
function isLeaf(node) {
	return 'value' in node;
}

/**
 * @param {Node} node
 * @returns {node is LeafBranch}
 */
function isExplodableBranch(node) {
	return isBranch(node) &&
			isLeaf(node.left) && isLeaf(node.right) &&
			level(node) > 4;
}

/**
 * @param {Node} node
 * @param {Omit<Leaf, 'parent'> | Omit<Branch, 'parent'>} replacement
 */
 function replace(node, replacement) {
	if (node.parent.left === node) {
		node.parent.left = { ...replacement, parent: node.parent };
	} else {
		node.parent.right = { ...replacement, parent: node.parent };
	}
}

/**
 * @param {Node} node
 * @returns {number}
 */
 function magnitude(node) {
	if (isLeaf(node)) { return node.value; }
	return 3 * magnitude(node.left) + 2 * magnitude(node.right);
}

/**
 * @param {Node} tree
 * @returns {Node[]}
 */
function flatten(tree) {
	if (isLeaf(tree)) { return [tree]; }

	return [
		...flatten(tree.left),
		tree,
		...flatten(tree.right),
	];
}

/**
 * @param {Node} node
 * @returns {number}
 */
function level(node) {
	return node.parent === undefined ? 1 : 1 + level(node.parent);
}

/**
 * @template T
 * @param {(x: T) => T} fn
 * @param {T} initial
 * @returns {T}
 */
function untilStable(fn, initial) {
	while (true) {
		const next = fn(initial);
		if (equals(initial, next)) { return next; }
		initial = next;
	}
}

/**
 * @returns {Node[]}
 */
function parseInput() {
	return [...readBlocksFromStdin()]
		.map(fromString);
}

/**
 * @param {string} string
 * @returns {Node}
 */
function fromString(string) {
	function toTree_(value, parent) {
		if (typeof value === 'number') {
			return { value, parent }
		}

		const node = { parent, left: undefined, right: undefined };
		node.left = toTree_(value[0], node);
		node.right = toTree_(value[1], node);
		return node;
	}

	return toTree_(JSON.parse(string), undefined);
}

/**
 * @param {Node} node
 * @returns {string}
 */
function toString(node) {
	if (isLeaf(node)) { return String(node.value); }
	return `[${toString(node.left)},${toString(node.right)}]`;
}

/**
 * @param {Node} node
 * @returns {Node}
 */
function clone(node) {
	return fromString(toString(node));
}
