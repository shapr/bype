use std::collections::HashMap;

type Pair = (u8, u8);

fn replace(source: &[u8], replacement: Pair, id: u8) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    let mut skip_next = false;

    for pair in std::iter::zip(source.iter(), source[1..].iter()) {
        let temp = (*pair.0, *pair.1); // 🔥 🔥
        if temp == replacement {
            buffer.push(id);
            skip_next = true;
        } else if skip_next {
            skip_next = false
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
    //print_hello_world();

    let input = "he he llo world AAA"; // include_bytes!("main.rs");
    let mut bytes: Vec<u8> = input.as_bytes().into();

    while let Some(candidate_pair) = most_frequent_pair(&bytes) {
        let mut table = HashMap::<Pair, u8>::new(); // Pair, Replacement

        if let Some(unused_byte) = get_unused_byte(&bytes) {
            // 🔥

            println!("Replace {candidate_pair:?} with {unused_byte:?}");

            table.insert(candidate_pair, unused_byte);
            println!("BYTES BEFORE {bytes:?}");
            bytes = replace(&bytes, candidate_pair, unused_byte);
            println!("BYTES AFTER {bytes:?}");
        }
    }

    //

    //get_unused_bytes(vec![]);
}

fn most_frequent_pair(bytes: &[u8]) -> Option<Pair> {
    let mut counts: HashMap<(u8, u8), u32> = HashMap::new();

    // Count pairs
    for pair in std::iter::zip(bytes.iter(), bytes[1..].iter()) {
        // println!("{pair:?}");

        let k = (*pair.0, *pair.1);
        let count = *counts.get(&k).unwrap_or(&0);
        counts.insert(k, count + 1);
    }
    //counts
    let (pair, count) = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap(); // 🔥

    if *count > 1 {
        Some(*pair)
    } else {
        None
    }
}

fn get_unused_byte(used_bytes: &[u8]) -> Option<u8> {
    // sort and deduplicate gives one of each byte used

    let mut used_bytes = used_bytes.to_owned();
    used_bytes.sort_unstable();
    used_bytes.dedup();

    // left join on all bytes
    (65..=91).find(|&x| !used_bytes.contains(&x))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_unused_bytes_bounds() {
//         let res = get_unused_bytes(vec![]);
//         assert!(res.contains(&std::u8::MIN));
//         assert!(res.contains(&std::u8::MAX));
//     }

//     #[test]
//     fn test_unused_bytes_empty() {
//         let res = get_unused_bytes((std::u8::MIN..=std::u8::MAX).collect());
//         println!("{:?}", res);
//         assert!(res.is_empty());
//     }
// }
