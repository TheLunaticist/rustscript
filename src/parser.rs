use crate::byte_crawler::ByteCrawler;

pub struct Parser<'a> {
	crawler: ByteCrawler<'a>
}

impl<'a> Parser<'a> {
	pub fn new(code_buf: &'a str) -> Self {
		Self {
			crawler: ByteCrawler::new(code_buf)
		}
	}
}