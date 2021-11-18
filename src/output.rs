use crate::data::APIData;
use crate::util;

pub fn print_output(data: APIData, url: &str, compact: bool) {
	let parent = data.parent.map_or(String::new(), |x| x + ".");

	println!("\n{}{} ({})", parent, data.name, data.internal_type);
	println!("\n{}", util::clean_description(&data.description));

	if let Some(props) = data.props {
		print_list(props, "Properties", true, compact);
	}

	if let Some(methods) = data.methods {
		print_list(methods, "Methods", true, compact);
	}

	if let Some(events) = data.events {
		print_list(events, "Events", true, compact);
	}

	if let Some(r#type) = data.r#type {
		println!("\nType:\n{}", r#type);
	}

	if let Some(params) = data.params {
		let params = params
			.iter()
			.map(|e| format!("{} ({}): {}", e.name, e.r#type, e.description))
			.collect();

		print_list(params, "Parameters", false, compact);
	}

	if let Some(returns) = data.returns {
		println!("\nReturns:\n{}", returns.r#type);
	}

	println!("\n -> View full docs: [{}]\n", url);
}

fn print_list(mut list: Vec<String>, name: &str, sort: bool, compact: bool) {
	list = list.into_iter().filter(|e| !e.starts_with('_')).collect();

	if sort {
		list.sort();
	}

	print!("\n{}:\n{}", name, if compact { "" } else { "  - " });
	println!("{}", list.join(if compact { ", " } else { "\n  - " }));
}
