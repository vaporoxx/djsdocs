use regex::Regex;
use std::borrow::Cow;
use std::fmt::Display;
use std::process;

pub fn clean_description(input: &str) -> Cow<str> {
	Regex::new(r"`([^`]*)`|\{@link ([^}]*)\}")
		.unwrap()
		.replace_all(input, "$1$2")
}

pub fn exit(message: impl Display) -> ! {
	eprintln!("{}: {}", env!("CARGO_PKG_NAME"), message);
	process::exit(1);
}

pub fn unwrap<T>(result: Result<T, impl Display>) -> T {
	result.unwrap_or_else(|err| exit(err))
}
