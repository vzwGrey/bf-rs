use std::fs::File;
use std::io::{stdin, Read};

#[derive(Debug)]
pub enum BrainfuckError {
	BucketOutOfRange,
	ValueOutOfRange,
	UnexpectedLoopEnd,
}

pub fn run(input: &str) -> Result<(), BrainfuckError> {
	let bytes: Vec<u8> = input.bytes().collect();
	let mut buckets: Vec<u8> = vec![0];
	let mut ptr = 0;
	let mut saved_ti = Vec::new();
	let mut stdin = stdin();

	let mut ti = 0;
	while ti < bytes.len() {
		match bytes[ti] {
			b'>' => {
				if ptr == (buckets.len() - 1) {
					buckets.push(0);
				}

				ptr += 1;
			},
			b'<' => {
				if ptr == 0 {
					return Err(BrainfuckError::BucketOutOfRange);
				}

				ptr -= 1;
			},
			b'+' => {
				if buckets[ptr] == 255 {
					return Err(BrainfuckError::ValueOutOfRange);
				}
				buckets[ptr] += 1;
			},
			b'-' => {
				if buckets[ptr] == 0 {
					return Err(BrainfuckError::ValueOutOfRange);
				}
				buckets[ptr] -= 1;
			}
			b'.' => print!("{}", buckets[ptr] as char),
			b',' => {
				let mut buf: [u8; 1] = [0];
				stdin.read(&mut buf).expect("to read from stdin");
				buckets[ptr] = buf[0];
			},
			b'[' => saved_ti.push(ti),
			b']' => {
				if buckets[ptr] != 0 {
					match saved_ti.last() {
						None => return Err(BrainfuckError::UnexpectedLoopEnd),
						Some(i) => ti = *i,
					}
				} else {
					if let None = saved_ti.pop() {
						return Err(BrainfuckError::UnexpectedLoopEnd);
					}
				}
			},
			_ => {},
		}

		ti += 1;
	}

	Ok(())
}

pub fn run_from_file(fp: &str) -> Result<(), BrainfuckError> {
	let mut file = File::open(fp).expect("to open file");
	let mut source = String::new();
	file.read_to_string(&mut source).expect("to read file contents");
	run(&source)
}
