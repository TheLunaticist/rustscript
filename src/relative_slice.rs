pub struct RelativeSlice {
	pos: usize,
	size: usize
}

impl RelativeSlice {
	pub fn new(pos: usize, size: usize) -> Self {
		Self { pos, size }
	}
	
	pub fn get_true_slice<'a>(&self, buf: &'a str) -> &'a str {
		return &buf[self.pos..(self.pos + self.size)]
	}
}