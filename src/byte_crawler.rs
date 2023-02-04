use crate::error::{Error, ErrorCode, MiscError};

pub struct ByteCrawler<'a> {
	buf: &'a [u8],
	pos: usize,
	char_pos: usize
}

impl<'a> ByteCrawler<'a> {
	pub fn new(buf: &'a str) -> Self {
		Self {
			buf: buf.as_bytes(),
			pos: 0,
			char_pos: 0
		}
	}
	
	pub fn get_next(&mut self) -> GetNextResult {
		const MASK_FIRST_BIT: u8 =  0b10000000;
		const MASK_SECOND_BIT: u8 = 0b01000000;
		const MASK_THIRD_BIT: u8 =  0b00100000;
		const MASK_FOURTH_BIT: u8 = 0b00010000;
		const MASK_FIFTH_BIT: u8 =  0b00001000;
		
		if self.pos >= self.buf.len() {
			return GetNextResult::ReachedBufferEnd
		}
		
		let first = self.buf[self.pos];
		if first & MASK_FIRST_BIT == 0 { //one byte
			self.pos += 1;
			return GetNextResult::GotChar(char::from(first))
		} 
		
		if first & MASK_SECOND_BIT == 1 {
			if first & MASK_THIRD_BIT == 0 { //two bytes
				//checking if there's enough space in the buffer
				if self.pos + 2 > self.buf.len() {
					return GetNextResult::ReachedBufferEnd
				}
				let mut res = u32::from(first & 0b00011111); //zeroing first three
				res <<= 6; //making space for 6 more bits
				res |= u32::from(self.buf[self.pos + 1] & 0b00111111); //copying 6 bits
				self.pos += 2;
				self.char_pos += 1;
				return GetNextResult::GotChar(char::from_u32(res).unwrap())
			} else if first & MASK_FOURTH_BIT == 0 { //three bytes
				if self.pos + 3 > self.buf.len() {
					return GetNextResult::Error(Error::new_tell(ErrorCode::Misc(MiscError::InvalidUTF8), self.char_pos))
				}
				let mut res = u32::from(first & 0b00001111); //zeroing first four
				res <<= 6; //making space for 6 more bits
				res |= u32::from(self.buf[self.pos + 1] & 0b00111111); //copying 6 bits
				res <<= 6; //making space for even more bits
				res |= u32::from(self.buf[self.pos + 2] & 0b00111111);
				self.pos += 3;
				self.char_pos += 1;
				return GetNextResult::GotChar(char::from_u32(res).unwrap());
			} else if first & MASK_FIFTH_BIT == 0 { //four bytes
				if self.pos + 4 > self.buf.len() {
					return GetNextResult::Error(Error::new_tell(ErrorCode::Misc(MiscError::InvalidUTF8), self.char_pos))
				}
				let mut res = u32::from(first & 0b00000111); //zeroing first five
				res <<= 6; //making space for 6 more bits
				res |= u32::from(self.buf[self.pos + 1] & 0b00111111); //copying 6 bits
				res <<= 6; //making space for even more bits
				res |= u32::from(self.buf[self.pos + 2] & 0b00111111);
				res <<= 6; //and even more
				res |= u32::from(self.buf[self.pos + 3] & 0b00111111);
				self.pos += 4;
				self.char_pos += 1;
				return GetNextResult::GotChar(char::from_u32(res).unwrap());
			}
		}
		
		return GetNextResult::Error(Error::new_tell(ErrorCode::Misc(MiscError::InvalidUTF8), self.char_pos))
	}
	
	pub fn get_pos(&self) -> usize {
		return self.pos
	}
	
	pub fn set_pos(&mut self, pos: usize) {
		self.pos = pos;
	}
	
	pub fn get_buf(&self) -> &[u8] {
		return self.buf
	}
	
	pub fn get_char_pos(&self) -> usize {
		return self.char_pos
	}
}

pub enum GetNextResult {
	GotChar(char),
	ReachedBufferEnd,
	Error(Error)
}