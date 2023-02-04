use crate::byte_crawler::{ByteCrawler, GetNextResult};
use crate::error::{Error};

use std::str;

pub struct Parser<'a> {
	crawler: ByteCrawler<'a>
}

impl<'a> Parser<'a> {
	pub fn new(code_buf: &'a str) -> Self {
		Self {
			crawler: ByteCrawler::new(code_buf)
		}
	}
	
	pub fn check_for_string(&mut self, string: &str) -> CheckForStringResult {
		let start_pos = self.crawler.get_pos();
		let mut on_first_char = true; 
		
		for schar in string.chars() {
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return CheckForStringResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					if on_first_char {
						self.crawler.set_pos(start_pos);
						return CheckForStringResult::StartedAtBufferEnd
					}
					self.crawler.set_pos(start_pos);
					return CheckForStringResult::FoundNothing
				},
				GetNextResult::GotChar(fchar) => {
					if schar != fchar {
						self.crawler.set_pos(start_pos);
						return CheckForStringResult::FoundNothing
					}
					
					on_first_char = false;
				}
			}
		}
		
		return CheckForStringResult::FoundIt
	}
	
	pub fn skip_whitespace(&mut self) -> SkipWhitespaceResult {
		loop {
			let last_pos = self.crawler.get_pos();
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return SkipWhitespaceResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					self.crawler.set_pos(last_pos);
					return SkipWhitespaceResult::ReachedBufferEnd
				},
				GetNextResult::GotChar(ch) => {
					if ch != ' ' && ch != '\n' && ch != '\r' {
						self.crawler.set_pos(last_pos);
						return SkipWhitespaceResult::DidIt
					}
				}
			}
		}
	}
	
	pub fn parse_lowercase_identifier(&mut self) -> ParseLowercaseIdentifierResult {
		let start_pos = self.crawler.get_pos();
		let mut num_chars = 0;
		loop {
			let last_pos = self.crawler.get_pos();
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return ParseLowercaseIdentifierResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					self.crawler.set_pos(last_pos);
					return ParseLowercaseIdentifierResult::ReachedBufferEnd
				},
				GetNextResult::GotChar(ch) => {
					match u32::from(ch) {
						97..=122 | 95 => {
							num_chars += 1;
						},
						_ => {
							self.crawler.set_pos(last_pos);
							if num_chars == 0 {
								return ParseLowercaseIdentifierResult::FoundNothing
							}
							unsafe {
								return ParseLowercaseIdentifierResult::GotIt(str::from_utf8_unchecked(&self.crawler.get_buf()[start_pos..start_pos + num_chars]))
							}
						}
					}
				}
			}
		}
	}
}

pub enum CheckForStringResult {
	Error(Error),
	StartedAtBufferEnd,
	FoundNothing,
	FoundIt
}

pub enum SkipWhitespaceResult {
	Error(Error),
	ReachedBufferEnd,
	DidIt
}

pub enum ParseLowercaseIdentifierResult<'a> {
	Error(Error),
	GotIt(&'a str),
	ReachedBufferEnd,
	FoundNothing
}