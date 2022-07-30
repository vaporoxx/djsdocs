use serde::Deserialize;

#[derive(Deserialize)]
pub struct APIData {
	pub functions: Option<Vec<Element>>,
	pub classes: Option<Vec<Element>>,
	pub typedefs: Option<Vec<Element>>,
}

#[derive(Deserialize)]
pub struct Element {
	pub name: String,
	pub description: Option<String>,
	pub props: Option<Vec<Element>>,
	pub methods: Option<Vec<Element>>,
	pub events: Option<Vec<Element>>,
}
