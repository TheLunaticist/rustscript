use crate::relative_slice::RelativeSlice;

pub struct Error {
	error_type: ErrorType,
	code: ErrorCode
}

impl Error {
	pub fn new_tell(code: ErrorCode) -> Self {
		Self {
			code: code,
			error_type: ErrorType::Tell
		}
	}
	
	pub fn new_show(code: ErrorCode, excerpt: Excerpt) -> Self {
		Self {
			code: code,
			error_type: ErrorType::Show(excerpt)
		}
	}
	
	pub fn print(&self) {
		match self.code {
			ErrorCode::InvalidUTF8 => {
				println!("Error: InvalidUTF8");
				println!("There's invalid UTF8 in the input.");
			},
			ErrorCode::NotImplemented => {
				println!("Error: NotImplemented");
				println!("A function was called that was not finished yet.");
			},
			ErrorCode::NothingFittingFound => {
				println!("Error: NothingFittingFound");
				println!("No fitting element was found.");
			},
			ErrorCode::FunctionNeverEnds => {
				println!("Error: FunctionNeverEnds");
				println!("The code ended before the function ended.");
			},
			ErrorCode::ExpectedIdentifier => {
				println!("Error: ExpectedIdentifier");
				println!("A function didn't have an (valid) identifier.");
			}
		}
		
		match &self.error_type {
			ErrorType::Tell => {},
			ErrorType::Show(excerpt) => {
				//excerpt.print();
			}
		}
	}
}

pub struct Excerpt{
	excerpt: RelativeSlice,
	mark_start: usize,
	mark_size: usize
}

impl Excerpt {
	fn new(buf: RelativeSlice, start: usize, size: usize) -> Self {
		Excerpt {
			excerpt: buf,
			mark_start: start,
			mark_size: size
		}
	}

	pub fn print(&self, buf: &str) {
		let true_slice = self.excerpt.get_true_slice(buf);
		
		print!("{}", true_slice);
		
		for c in true_slice.chars() {
			print!("{c}");
		}
		
		for _ in 0..(self.mark_start) {
			print!("{}", ' ');
		}
		println!("");
		
		for _ in 0..(self.mark_size) {
			print!("^");
		}
		println!("");
	}
}

pub enum ErrorCode {
	InvalidUTF8,
	NotImplemented,
	NothingFittingFound,
	FunctionNeverEnds,
	ExpectedIdentifier
}

enum ErrorType {
	Tell,
	Show(Excerpt)
}
