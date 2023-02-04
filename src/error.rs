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
	
	pub fn print(&self, buf: &str) {
		match &self.code {
			/* Misc */
			ErrorCode::Misc(error) => {
				match error {
					MiscError::InvalidUTF8 => {
						println!("Error: InvalidUTF8");
						println!("Ran into invalid UTF8 during execution.");
					},
					MiscError::NotImplemented => {
						println!("Error: NotImplemented");
						println!("Generic error for indicating unfinishedness.");
					}
				}
			},
			/* Global */
			ErrorCode::Global(error) => {
				match error {
					GlobalError::NothingFittingFound => {
						println!("Error: NothingFittingFound");
						println!("In the global space nothing did match.");
					}
				}
			},
			/* Function */
			ErrorCode::Function(error) => {
				match error {
					FunctionError::ExpectedIdentifier => {
						println!("Error: ExpectedIdentifier");
						println!("While parsing we found that there was no identifier after a fn keyword.");
					},
					FunctionError::FunctionNeverEnds => {
						println!("Error: FunctionNeverEnds");
						println!("While parsing a function we hit the buffer end.");
					}
				}
			},
			/* Operation */
			ErrorCode::Operation(error) => {
				match error {
					OperationError::CallNotClosed => {
						println!("Error: CallNotClosed");
						println!("What was obviously a function call didn't have any closing () thingies");
					},
					OperationError::CallIsNotFinished => {
						println!("Error: CallIsNotFinished");
						println!("A call to a function was started but the program ended suddenly.");
					}
				}
			}
		}
		
		match &self.error_type {
			ErrorType::Tell => {},
			ErrorType::Show(excerpt) => {
				excerpt.print(buf);
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
	Misc(MiscError),
	Global(GlobalError),
	Function(FunctionError),
	Operation(OperationError)
}

pub enum MiscError {
	InvalidUTF8,
	NotImplemented
}

pub enum GlobalError {
	NothingFittingFound
}

pub enum FunctionError {
	ExpectedIdentifier,
	FunctionNeverEnds
}

pub enum OperationError {
	CallNotClosed,
	CallIsNotFinished
}


enum ErrorType {
	Tell,
	Show(Excerpt)
}
