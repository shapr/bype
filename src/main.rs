use bype::lib::print_hello_world;
use std::fs::File;
use std::io::Read;

fn main() {
    print_hello_world();
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn get_unused_bytes(mut used_bytes: Vec<u8>) -> Vec<u8> {
    // sort and deduplicate gives one of each byte used
    used_bytes.sort_unstable();
    used_bytes.dedup();
    // left join on all bytes
    let mut all_bytes: Vec<u8> = vec![];
    for b in std::u8::MIN..=std::u8::MAX {
	all_bytes.push(b);
    }
    all_bytes
	.iter()
	.filter(|&x| used_bytes.contains(x))
	.cloned() // what the heck is this?
	.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unused_bytes_bounds() {
	let res = get_unused_bytes(vec![]);
	assert!(res.contains(&std::u8::MIN));
	assert!(res.contains(&std::u8::MAX));
    }

    #[test]
    fn test_unused_bytes_empty() {
	let res = get_unused_bytes((std::u8::MIN..=std::u8::MAX).collect());
	println!("{:?}", res);
	assert!(res.is_empty());
    }
}
