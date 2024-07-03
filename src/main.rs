use std::collections::HashMap;

type Pair = (u8, u8);

fn replace(source: &[u8], replacement: Pair, id: u8) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    let mut skip_next = false;

    for pair in std::iter::zip(source.iter(), source[1..].iter()) {
	if skip_next {
	    skip_next = false;
	    continue;
	}
	if (*pair.0, *pair.1) == replacement {
	    buffer.push(id);
	    skip_next = true;
	} else {
	    buffer.push(*pair.0);
	}
    }
    if !skip_next {
	buffer.push(*source.last().unwrap());
    }
    buffer
}

fn main() {
    let input = "he hello world AAAX";
    let bytes: Vec<u8> = input.as_bytes().into();

    let table = build_table(bytes);
    for row in table {
	println!(
	    "{:?} replaces {:?} {:?}",
	    char::from(row.1),
	    char::from(row.0 .0),
	    char::from(row.0 .1)
	);
    }
}

fn build_table(mut bytes: Vec<u8>) -> Vec<((u8, u8), u8)> {
    // A list of Replacements and Counts.
    // Pair = (a,a), Replacement = H
    let mut table: Vec<(Pair, u8)> = Vec::new();
    // Pair, Replacement

    while let Some(candidate_pair) = most_frequent_pair(&bytes) {
	if let Some(unused_byte) = get_unused_byte(&bytes) {
	    println!("Replace {candidate_pair:?} with {unused_byte:?}");

	    table.push((candidate_pair, unused_byte));

	    println!("BYTES BEFORE {:?}", String::from_utf8(bytes.clone()));
	    bytes = replace(&bytes, candidate_pair, unused_byte);
	    println!("BYTES AFTER {:?}", String::from_utf8(bytes.clone()));
	}
    }
    table
}

fn most_frequent_pair(bytes: &[u8]) -> Option<Pair> {
    let mut counts: HashMap<(u8, u8), u32> = HashMap::new();

    // Count pairs
    for pair in std::iter::zip(bytes.iter(), bytes[1..].iter()) {
	let k = (*pair.0, *pair.1);
	let count = *counts.get(&k).unwrap_or(&0);
	counts.insert(k, count + 1);
    }
    counts
	.iter()
	.max_by(|a, b| a.1.cmp(b.1))
	.and_then(|(pair, count)| if *count > 1 { Some(*pair) } else { None })
}

fn get_unused_byte(used_bytes: &[u8]) -> Option<u8> {
    // sort and deduplicate gives one of each byte used

    let mut used_bytes = used_bytes.to_owned();
    used_bytes.sort();
    used_bytes.dedup();

    // left join on all bytes
    (65..=91).find(|&x| !used_bytes.contains(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
	let res = replace(b"AA", (b'A', b'A'), b'B'); // cooool
	assert_eq!(res, vec![b'B']);
    }
}
