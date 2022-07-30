use crate::data::{APIData, Element};
use crate::url::ElementType;
use crate::util;

pub fn print_data(data: APIData, url: &str) {
	if let Some(functions) = data.functions {
		let functions = functions.into_iter().map(list_element);
		print_iter(functions, "Functions", true);
	}

	if let Some(classes) = data.classes {
		let classes = classes.into_iter().map(list_element);
		print_iter(classes, "Classes", true);
	}

	if let Some(typedefs) = data.typedefs {
		let typedefs = typedefs.into_iter().map(list_element);
		print_iter(typedefs, "Typedefs", true);
	}

	println!("\n -> View full docs: [{}]\n", url);
}

pub fn print_element(element: Element, url: &str, element_type: ElementType) {
	println!("\n{} ({})", element.name, element_type.as_str());

	if let Some(description) = element.description {
		println!("\n{}", util::clean_description(description, false));
	}

	if let Some(props) = element.props {
		print_iter(props.into_iter().map(list_element), "Properties", true);
	}

	if let Some(methods) = element.methods {
		print_iter(methods.into_iter().map(list_element), "Methods", true);
	}

	if let Some(events) = element.events {
		print_iter(events.into_iter().map(list_element), "Events", true);
	}

	println!("\n -> View full docs: [{}]\n", url);
}

fn list_element(element: Element) -> String {
	match element.description {
		Some(description) => element.name + ": " + &util::clean_description(description, true),
		None => element.name,
	}
}

fn print_iter(iter: impl Iterator<Item = String>, name: &str, sort: bool) {
	let mut vec: Vec<_> = iter.filter(|e| !e.starts_with('_')).collect();

	if sort {
		vec.sort();
	}

	println!("\n{}:\n  - {}", name, vec.join("\n  - "));
}
