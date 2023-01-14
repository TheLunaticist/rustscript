pub struct Error<'a> {
	error_type: ErrorType<'a>,
	code: ErrorCode
}

impl<'a> Error<'a> {
	pub fn new_tell(code: ErrorCode) -> Self {
		Self {
			code: code,
			error_type: ErrorType::Tell
		}
	}
	
	pub fn new_show(code: ErrorCode, excerpt: Excerpt<'a>) -> Self {
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
		
		match self.error_type {
			ErrorType::Tell => {},
			ErrorType::Show(excerpt) => {
				excerpt.print();
			}
		}
	}
}

struct Excerpt<'a> {
	excerpt: &'a str,
	mark_start: usize,
	mark_size: usize
}

impl<'a> Excerpt<'a> {
	pub fn new(buf: &str, start: usize, size: usize) -> Self {
		Excerpt {
			excerpt: buf,
			mark_start: start,
			mark_size: size
		}
	}
	
	pub fn print(&self) {
		print!("{}", self.excerpt);
		
		for c in self.excerpt.chars() {
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

enum ErrorType<'a> {
	Tell,
	Show(Excerpt<'a>)
}
