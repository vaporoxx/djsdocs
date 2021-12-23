mod args;
mod data;
mod output;
mod url;
mod util;

use data::APIResponse;
use reqwest::blocking as reqwest;

fn main() {
	let args = args::parse_args();

	let query = args.positionals.join(".").replace('#', ".");
	let source = args.options.get("src").map_or("stable", |e| e);

	let compact = args.options.contains_key("compact") || args.flags.contains(&'c');
	let force = args.options.contains_key("force") || args.flags.contains(&'f');

	let url = format!(
		"https://djsdocs.sorta.moe/v2?src={}&force={}&q={}",
		source, force, query,
	);

	let response = util::unwrap(reqwest::get(url));
	let parsed = util::unwrap(response.json::<APIResponse>());

	match parsed {
		APIResponse::Element(data) => {
			let url = util::unwrap(url::parse_url(&data, source));
			output::print_element(data, &url, compact);
		}
		APIResponse::List(data) => output::print_list(data, compact),
		APIResponse::Error(error) => util::exit(error.message),
	}
}
