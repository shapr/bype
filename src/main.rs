use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

#[derive(Eq, Clone, Copy, Hash, PartialEq)]
struct Pair(u8, u8);

type Count = u32;

impl Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", char::from(self.0), char::from(self.1))
    }
}

/// Counts of all pairs in bytes
struct CountsTable(HashMap<Pair, Count>);

impl CountsTable {
    /// Create a frequency table from input bytes
    fn new(bytes: &[u8]) -> Self {
        let mut counts: HashMap<Pair, Count> = HashMap::new();

        // Count pairs
        for pair in std::iter::zip(bytes.iter(), bytes[1..].iter()) {
            counts
                .entry(Pair(*pair.0, *pair.1))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        CountsTable(counts)
    }

    /// Most frequent pair
    fn top_pair(&self) -> Option<Pair> {
        self.0
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .and_then(|(pair, count)| if *count > 1 { Some(*pair) } else { None })
    }

    /// Decrease the count associated with a pair
    fn decrease(&mut self, pair: &Pair, count: Count) {
        println!("Decrease {:?} by {}", pair, count);
        self.0.entry(*pair).and_modify(|counter| *counter -= count);
    }
}

/// Replace `replacement` in `source` with `id`
///
/// return (new buffer, number of replacements)
fn replace(source: &[u8], replacement: Pair, id: u8) -> (Vec<u8>, Count) {
    let mut buffer = Vec::<u8>::new();
    let mut skip_next = false;
    let mut replacements = 0;

    for pair in std::iter::zip(source.iter(), source[1..].iter()) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if Pair(*pair.0, *pair.1) == replacement {
            buffer.push(id);
            replacements += 1;
            skip_next = true;
        } else {
            buffer.push(*pair.0);
        }
    }
    if !skip_next {
        buffer.push(*source.last().unwrap());
    }

    (buffer, replacements)
}

fn compress(input: &str) -> (Vec<u8>, Vec<(Pair, u8)>) {
    let mut bytes: Vec<u8> = input.as_bytes().into();
    let mut count;
    let mut table: Vec<(Pair, u8)> = Vec::new(); // A list of Replacements and Counts.
    let mut counts = CountsTable::new(&bytes); // Frequency of each pair

    while let Some(candidate_pair) = counts.top_pair() {
        if let Some(unused_byte) = get_unused_byte(&bytes) {
            println!("Before step {:?}", std::str::from_utf8(&bytes).unwrap());
            println!(
                "Replace '{candidate_pair:?}' -> {:?}",
                char::from(unused_byte)
            );

            table.push((candidate_pair, unused_byte));

            (bytes, count) = replace(&bytes, candidate_pair, unused_byte);

            counts.decrease(&candidate_pair, count);
            println!("After step {:?}\n", std::str::from_utf8(&bytes).unwrap());
        }
    }

    (bytes, table)
}

fn main() {
    let input = "Hel Hello World AAA XYZ"; // include_str!("main.rs");
    let (output, table) = compress(input);

    println!("Output = {:?}", std::str::from_utf8(&output).unwrap());
    println!(
        "Compression ratio = {:?}%",
        100 * output.len() / input.len()
    );
    println!("\nReplacement Table: ");

    for (pair, replacement) in table {
        println!("Replace '{:?}' -> {:?} ", pair, replacement);
    }
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
        let (res, _count) = replace(b"AA", Pair(b'A', b'A'), b'B');
        assert_eq!(res, vec![b'B']);
    }
}
