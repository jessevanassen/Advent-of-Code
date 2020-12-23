import { map } from '../../lib/fp/generators';

export interface Node<T> {
	value: T;
	next: Node<T>;
}

export function toLinkedList<T>(items: Iterable<T>): Node<T> {
	type TempNode = { value: T; next?: TempNode };

	const iterator = items[Symbol.iterator]();
	let node: TempNode = { value: iterator.next().value };

	const first = node;

	for (let { value, done } = iterator.next(); !done; { value, done } = iterator.next()) {
		const newNode = { value };
		node.next = newNode;
		node = newNode;
	}
	node.next = first;

	return first as Node<T>;
}

export function* iterate<T>(node: Node<T>): IterableIterator<Node<T>> {
	let currentNode: Node<T> | undefined = node;

	do {
		yield currentNode;
		currentNode = currentNode.next;
	} while (currentNode && currentNode !== node);
}

export function values<T>(node: Node<T>): IterableIterator<T> {
	return map(
		(node: Node<T>) => node.value,
	)(iterate(node));
}

/**
 * Insert the given values after the given node.
 * The given node is mutated.
 */
export function insert<T>(node: Node<T>, ...values: T[]): void {
	for (let i = values.length - 1; i >= 0; i--) {
		node.next = { value: values[i], next: node.next };
	}
}

/**
 * Removes the item after the passed node from the list.
 * The given node is mutated.
 */
export function pop<T>(node: Node<T>): T {
	const value = node.next.value;
	node.next = node.next.next;
	return value;
}
