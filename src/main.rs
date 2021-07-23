mod args;
mod data;
mod output;
mod url;
mod util;

use data::APIData;
use reqwest::blocking as reqwest;

fn main() {
	let args = args::parse();

	let source = args
		.options
		.get("src")
		.unwrap_or(&"stable".to_owned())
		.to_owned();

	let force = args.options.contains_key("force");

	let url = format!(
		"https://djsdocs.sorta.moe/v2?src={}&force={}&q={}",
		source,
		force,
		args.query.replace('#', "."),
	);

	let response = util::unwrap(reqwest::get(url));
	let parsed = util::unwrap(response.json::<APIData>());

	let data = data::parse(parsed, &source);
	let compact = args.options.contains_key("compact");

	output::print(data, compact);
}
