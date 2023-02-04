mod function;
mod byte_crawler;
mod parser;
mod error;
mod relative_slice;
mod operation;

use crate::function::{parse_function, ParseFunctionResult};
use crate::parser::Parser;
use crate::error::{Error, ErrorCode, GlobalError};

fn main() {
	let code_buf = "fn test() {}";
	
	match run_rss(code_buf) {
		RunRssResult::Finished => {
			println!("successfully executed code");
		},
		RunRssResult::Error(error) => {
			error.print();
		}
	}
}

fn run_rss(code_buf: &str) -> RunRssResult {
	let mut parser = Parser::new(code_buf);
	
	loop {
		match parse_function(&mut parser) {
			ParseFunctionResult::Error(error) => {
				return RunRssResult::Error(error)
			},
			ParseFunctionResult::StartedAtBufferEnd => {
				return RunRssResult::Finished
			},
			ParseFunctionResult::NoFunction => {},
			ParseFunctionResult::GotFunction(name, func) => {
				println!("Function was parsed but not saved. Name: {}", &name);
				continue
			}
		}
		
		return RunRssResult::Error(Error::new_tell(ErrorCode::Global(GlobalError::NothingFittingFound)))
	}
}

enum RunRssResult {
	Finished,
	Error(Error)
}