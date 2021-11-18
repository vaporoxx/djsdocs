mod args;
mod data;
mod output;
mod url;
mod util;

use data::APIResponse;
use reqwest::blocking as reqwest;

fn main() {
	let args = args::parse_args();

	if args.positionals.is_empty() {
		util::exit("no query provided")
	}

	let source = args.options.get("src").map_or("stable", |e| e);
	let force = args.options.contains_key("force") || args.flags.contains(&'f');

	let url = format!(
		"https://djsdocs.sorta.moe/v2?src={}&force={}&q={}",
		source,
		force,
		args.positionals.join(".").replace('#', "."),
	);

	let response = util::unwrap(reqwest::get(url));
	let parsed = util::unwrap(response.json::<APIResponse>());

	let data = util::unwrap(parsed.try_into());
	let url = util::unwrap(url::parse_url(&data, source));
	let compact = args.options.contains_key("compact") || args.flags.contains(&'c');

	output::print_output(data, &url, compact);
}
