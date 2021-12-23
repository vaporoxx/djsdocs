use regex::Regex;
use std::fmt::Display;
use std::process;

pub fn clean_description(input: &str) -> String {
	let newlines = input.replace('\n', " ");

	let info = Regex::new("<info>(.*?)</info>")
		.unwrap()
		.replace_all(&newlines, "Info: $1");

	let warn = Regex::new("<warn>(.*?)</warn>")
		.unwrap()
		.replace_all(&info, "Warning: $1");

	Regex::new(r"`(.*?)`|\{@link (.*?)\}|<p>(.*?)</p>")
		.unwrap()
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
