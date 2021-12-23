use crate::data::{ElementData, ListData};
use crate::util;

pub fn print_element(data: ElementData, url: &str, compact: bool) {
	let parent = data.parent.map_or(String::new(), |x| x + ".");

	println!("\n{}{} ({})", parent, data.name, data.internal_type);
	println!("\n{}", util::clean_description(&data.description));

	if let Some(props) = data.props {
		print_vec(props, "Properties", true, compact);
	}

	if let Some(methods) = data.methods {
		print_vec(methods, "Methods", true, compact);
	}

	if let Some(events) = data.events {
		print_vec(events, "Events", true, compact);
	}

	if let Some(r#type) = data.r#type {
		println!("\nType:\n{}", r#type);
	}

	if let Some(params) = data.params {
		let params = params
			.iter()
			.map(|e| format!("{} ({}): {}", e.name, e.r#type, e.description))
			.collect();

		print_vec(params, "Parameters", false, compact);
	}

	if let Some(returns) = data.returns {
		println!("\nReturns:\n{}", returns.r#type);
	}

	println!("\n -> View full docs: [{}]\n", url);
}

pub fn print_list(data: ListData, compact: bool) {
	let classes = data.classes.into_iter().map(|e| e.name).collect();
	let interfaces = data.interfaces.into_iter().map(|e| e.name).collect();
	let typedefs = data.typedefs.into_iter().map(|e| e.name).collect();

	print_vec(classes, "Classes", true, compact);
	print_vec(interfaces, "Interfaces", true, compact);
	print_vec(typedefs, "Typedefs", true, compact);
}

fn print_vec(mut vec: Vec<String>, name: &str, sort: bool, compact: bool) {
	vec = vec.into_iter().filter(|e| !e.starts_with('_')).collect();

	if sort {
		vec.sort();
	}

	print!("\n{}:\n{}", name, if compact { "" } else { "  - " });
	println!("{}", vec.join(if compact { ", " } else { "\n  - " }));
}
