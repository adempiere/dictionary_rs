use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{find, get_by_id, IndexDocument}, models::get_index_name};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct FormDocument {
	pub document: Option<Form>
}

#[derive(Serialize, Debug, Clone)]
pub struct FormResponse {
	pub form: Option<Form>
}

#[derive(Serialize, Debug, Clone)]
pub struct FormsListResponse {
	pub forms: Option<Vec<Form>>
}

impl Default for FormResponse {
	fn default() -> Self {
		FormResponse {
			form: None
		}
	}
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Form {
	pub uuid: Option<String>,
	pub id: Option<String>,
	pub internal_id: Option<i32>,
	pub file_name: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>,
	pub is_active: Option<bool>,
	pub is_beta_functionality: Option<bool>,
	//	Index
	pub index_value: Option<String>,
	pub language: Option<String>,
	pub client_id: Option<String>,
	pub role_id: Option<String>,
	pub user_id: Option<String>
}

impl Default for Form {
	fn default() -> Self {
		Self {
			uuid: None,
			id: None,
			internal_id: None,
			file_name: None,
			name: None,
			description: None,
			help: None,
			is_active: None,
			is_beta_functionality: None,
			//	Index
			index_value: None,
			language: None,
			client_id: None,
			role_id: None,
			user_id: None
		}
	}
}

impl Form {
	pub fn from_id(_id: Option<String>) -> Self {
		let mut form: Form = Form::default();
		form.id = _id;
		form
	}

	pub fn to_string(&self) -> String {
		format!("Form: UUID {:?}, ID {:?}, Name {:?}, Index: {:?}", self.uuid, self.internal_id, self.name, self.index_value)
	}
}

impl IndexDocument for Form {
	fn mapping(self: &Self) -> serde_json::Value {
		json!({
			"mappings" : {
				"properties" : {
					"uuid" : { "type" : "keyword" },
					"id" : { "type" : "keyword" },
					"internal_id" : { "type" : "integer" },
					"file_name" : { "type" : "keyword" },
					"name" : { "type" : "text" },
					"description" : { "type" : "text" },
					"help" : { "type" : "text" }
				}
			}
		})
	}

	fn data(self: &Self) -> serde_json::Value {
		json!(self)
	}

	fn id(self: &Self) -> String {
		self.id.to_owned().unwrap_or_else(|| {
			log::error!("{}", self.to_string());
			"".to_string()
		})
	}

	fn index_name(self: &Self) -> String {
		match &self.index_value {
			Some(value) => value.to_string(),
			None => "forms".to_string(),
		}
	}

	fn find(self: &Self, _search_value: String) -> serde_json::Value {
		let mut query: String = "*".to_owned();
		query.push_str(&_search_value.to_owned());
		query.push_str(&"*".to_owned());

		json!({
			"query": {
				"query_string": {
				"query": query
				}
			}
		})
	}
}

pub async fn form_from_id(_id: Option<String>, _language: Option<&String>, _dictionary_code: Option<&String>) -> Result<Form, String> {
	if _id.is_none() || _id.as_deref().map_or(false, |s| s.trim().is_empty()) {
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Form Identifier is Mandatory").to_string()
		);
	}
	let mut _document: Form = Form::from_id(_id);

	let _index_name: String = match get_index_name("form".to_string(), _language, _dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(error.to_string())
		}
	};
	log::info!("Index to search {:}", _index_name);

	_document.index_value = Some(_index_name);
	let _form_document: &dyn IndexDocument = &_document;
	match get_by_id(_form_document).await {
		Ok(value) => {
			let form: Form = serde_json::from_value(value).unwrap();
			log::info!("Finded Form Value: {:?}", form.id);
			// Ok(FormResponse {
			// 	form: Some(form)
			// })
			Ok(
				form
			)
		},
		Err(error) => {
			log::error!("{}", error);
			Err(error)
		},
	}
}

pub async fn forms(_language: Option<&String>, _search_value: Option<&String>, _dictionary_code: Option<&String>) -> Result<FormsListResponse, std::io::Error> {
	let _search_value: String = match _search_value {
		Some(value) => value.clone(),
		None => "".to_owned()
	};

	//  Find index
	let _index_name: String = match get_index_name("form".to_string(),_language, _dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};
	log::info!("Index to search {:}", _index_name);

	let mut _document: Form = Form::default();
	_document.index_value = Some(_index_name);
	let _forms_document: &dyn IndexDocument = &_document;
	match find(_forms_document, _search_value, 0, 10).await {
		Ok(values) => {
			let mut forms_list: Vec<Form> = vec![];
			for value in values {
				let form: Form = serde_json::from_value(value).unwrap();
				forms_list.push(form.to_owned());
			}
			Ok(FormsListResponse {
				forms: Some(forms_list)
			})
		},
		Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
	}
}
