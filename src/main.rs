use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

#[derive(Eq, Ord, Clone, Copy, Hash, PartialEq, PartialOrd)]
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
        // Sort for this to be deterministic
        let mut counts = self.0.iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| a.0.cmp(b.0));
        counts
            .into_iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .and_then(|(pair, count)| if *count > 1 { Some(*pair) } else { None })
    }
}

/// Replace `replacement` in `source` with `id`
///
/// return (new buffer, number of replacements)
fn replace(source: &[u8], replacement: Pair, id: u8) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    let mut skip_next = false;

    for pair in std::iter::zip(source.iter(), source[1..].iter()) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if Pair(*pair.0, *pair.1) == replacement {
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

fn compress(input: &str) -> (Vec<u8>, Vec<(Pair, u8)>) {
    let mut bytes: Vec<u8> = input.as_bytes().into();
    let mut table: Vec<(Pair, u8)> = Vec::new(); // A list of Replacements and Counts.

    // In theory, the table needs to be computed only once but that's work.
    while let Some(candidate_pair) = CountsTable::new(&bytes).top_pair() {
        if let Some(unused_byte) = get_unused_byte(&bytes) {
            // println!("Before step {:?}", std::str::from_utf8(&bytes).unwrap());
            println!(
                "Replace {candidate_pair:?} -> {:?}",
                char::from(unused_byte)
            );

            table.push((candidate_pair, unused_byte));

            bytes = replace(&bytes, candidate_pair, unused_byte);

            // println!("After step {:?}\n", std::str::from_utf8(&bytes).unwrap());
        }
    }

    (bytes, table)
}

fn main() {
    let input = "He Hello World AAA XYZ"; // include_str!("main.rs");
    let (output, table) = compress(input);

    println!("Output = {:?}", std::str::from_utf8(&output).unwrap());
    println!(
        "Compression ratio = {:?}%",
        100 * output.len() / input.len()
    );
    println!("\nReplacement Table: ");

    for (pair, replacement) in table {
        println!("Replace '{:?}' -> {:?} ", pair, char::from(replacement));
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
        let res = replace(b"AA", Pair(b'A', b'A'), b'B');
        assert_eq!(res, vec![b'B']);
    }

    #[test]
    fn test_compress() {
        let (out, table) = compress("He Hello World AAA XYZ");
        assert_eq!(
            table,
            vec![
                (Pair(b'H', b'e'), b'B'), //
                (Pair(b'A', b'A'), b'C'),
            ]
        );
        assert_eq!(out, "B Bllo World CA XYZ".as_bytes());
    }
}
