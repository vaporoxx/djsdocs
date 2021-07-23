use regex::Regex;
use std::fmt::Display;
use std::process;

pub fn clean(input: &str) -> String {
	let regexes = [
		Regex::new(r"`([^`]*)`").unwrap(),
		Regex::new(r"\{@link ([^}]*)\}").unwrap(),
	];

	regexes.iter().fold(input.to_owned(), |acc, regex| {
		regex.replace_all(&acc, "$1").to_string()
	})
}

pub fn exit(message: impl Display) -> ! {
	eprintln!("{}: {}", env!("CARGO_PKG_NAME"), message);
	process::exit(1);
}

pub fn unwrap<T>(result: Result<T, impl Display>) -> T {
	result.unwrap_or_else(|err| exit(err))
}
