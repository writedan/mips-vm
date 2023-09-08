#[derive(Debug)]
pub struct LexErr {
	pub line: usize,
	pub character: usize,
	pub len: usize, // these three values indicate the line and character indexes, as well as the length of the impacted sequence
				// this is used in printing error messages
	pub msg: String
}