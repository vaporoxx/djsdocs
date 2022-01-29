use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParameterData {
	pub name: String,
	pub description: String,
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct ReturnData {
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct ElementData {
	pub name: String,
	pub internal_type: String,
	pub description: Option<String>,
	pub parent: Option<String>,
	pub props: Option<Vec<String>>,
	pub methods: Option<Vec<String>>,
	pub events: Option<Vec<String>>,
	pub r#type: Option<String>,
	pub params: Option<Vec<ParameterData>>,
	pub returns: Option<ReturnData>,
}

#[derive(Deserialize)]
pub struct ListElementData {
	pub name: String,
}

#[derive(Deserialize)]
pub struct ListData {
	pub classes: Vec<ListElementData>,
	pub typedefs: Option<Vec<ListElementData>>,
}

#[derive(Deserialize)]
pub struct APIError {
	pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum APIResponse {
	Element(ElementData),
	List(ListData),
	Error(APIError),
}
