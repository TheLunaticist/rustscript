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
			ErrorType::Tell => {}
		}
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
	Tell
}
