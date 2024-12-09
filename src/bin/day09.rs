use std::{io::stdin, iter, ops::Range};

use aoc2024::slice::SliceExt;
use itertools::{Either, Itertools};

fn main() {
	let patterns = parse_input();
	println!("Part 1: {}", checksum(&defrag_bytes(&patterns)));
	println!("Part 2: {}", checksum(&defrag_blocks(&patterns)));
}

fn checksum(data: &[Option<u16>]) -> u64 {
	data.iter()
		.enumerate()
		.filter_map(|(i, v)| v.map(|v| (i, v)))
		.fold(0, |acc, (index, value)| acc + index as u64 * value as u64)
}

fn defrag_bytes(blocks: &[Option<u16>]) -> Vec<Option<u16>> {
	let mut filled_blocks = blocks.iter().copied().flatten();

	blocks
		.iter()
		.map(|block| {
			if block.is_some() {
				filled_blocks.next()
			} else {
				filled_blocks.next_back()
			}
		})
		.collect()
}

fn defrag_blocks(blocks: &[Option<u16>]) -> Vec<Option<u16>> {
	let mut blocks = blocks.to_vec();

	let (filled_blocks, mut empty_blocks): (Vec<_>, Vec<_>) =
		block_ranges(&blocks).partition_map(|(v, range)| match v {
			Some(_) => Either::Left(range),
			None => Either::Right(range),
		});

	for filled_range in filled_blocks.into_iter().rev() {
		if let Some(empty_range) = empty_blocks
			.iter_mut()
			.take_while(|empty_range| empty_range.start < filled_range.start)
			.find(|empty_range| empty_range.len() >= filled_range.len())
		{
			blocks.swap_chunks(filled_range.start, empty_range.start, filled_range.len());
			empty_range.start += filled_range.len();
		}
	}

	blocks
}

fn block_ranges(blocks: &[Option<u16>]) -> impl Iterator<Item = (Option<u16>, Range<usize>)> + '_ {
	let mut i = 0;

	iter::from_fn(move || {
		if i >= blocks.len() {
			return None;
		}

		let start = i;
		let end = blocks
			.iter()
			.enumerate()
			.skip(start)
			.find_map(|(i, &x)| (x != blocks[start]).then_some(i))
			.unwrap_or(blocks.len());

		i = end;

		Some((blocks[start], start..end))
	})
}

fn parse_input() -> Vec<Option<u16>> {
	let line = stdin().lines().next().unwrap().unwrap();
	let lengths = line.bytes().map(|b| (b - b'0') as usize);
	lengths
		.enumerate()
		.flat_map(|(i, len)| {
			let v = (i % 2 == 0).then_some((i / 2) as u16);
			iter::repeat_n(v, len)
		})
		.collect()
}
