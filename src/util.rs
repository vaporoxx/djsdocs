use regex::{Regex, RegexBuilder};
use std::fmt::Display;
use std::process;

fn regex(pattern: &str) -> Regex {
	RegexBuilder::new(pattern).dot_matches_new_line(true).build().unwrap()
}

pub fn clean_description(mut input: String, trim: bool) -> String {
	if trim && input.lines().count() > 1 {
		input = format!("{} [...]", input.lines().next().unwrap());
	}

	let info = regex("<info>(.*?)</info>").replace_all(&input, "Info: $1");
	let warn = regex("<warn>(.*?)</warn>").replace_all(&info, "Warning: $1");

	regex(r"`(.*?)`|\{@link (.*?)\}|<p>(.*?)</p>")
		.replace_all(&warn, "$1$2$3")
		.into_owned()
}

pub fn exit(message: impl Display) -> ! {
	eprintln!("{}: {}", env!("CARGO_PKG_NAME"), message);
	process::exit(1);
}

pub fn unwrap<T>(result: Result<T, impl Display>) -> T {
	result.unwrap_or_else(|err| exit(err))
}
