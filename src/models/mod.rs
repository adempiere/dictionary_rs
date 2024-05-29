pub mod browser;
pub mod form;
pub mod menu;
pub mod process;
pub mod window;
pub mod generic;

use serde::{Deserialize, Serialize};
use salvo::prelude::*;

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Metadata {
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
}

fn default_index(_index_name: String) -> String {
	let mut _index_to_find: String = _index_name.to_owned();
	_index_to_find.to_lowercase()
}

fn language_index(_index_name: String, _language: Option<&String>) -> String {
	let mut _index_to_find: String = default_index(_index_name);
	if let Some(language) = _language {
		_index_to_find.push_str("_");
		_index_to_find.push_str(language);
	}
	_index_to_find.to_lowercase()
}

fn client_index(_index_name: String, _language: Option<&String>, _client_id: Option<&String>) -> String {
	let mut _index_to_find: String = language_index(_index_name, _language);
	if let Some(client_id) = _client_id {
		if let Ok(id) = client_id.parse::<i32>() {
			if id >= 0 {
				_index_to_find.push_str("_");
				_index_to_find.push_str(client_id);
			}
		}
	}
	_index_to_find.to_lowercase()
}

fn role_index(_index_name: String, _language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>) -> String {
	let mut _index_to_find: String = client_index(_index_name, _language, _client_id);
	if let Some(role_id) = _role_id {
		if let Ok(id) = role_id.parse::<i32>() {
			if id >= 0 {
				_index_to_find.push_str("_");
				_index_to_find.push_str(role_id);
			}
		}
	}
	_index_to_find.to_lowercase()
}

fn user_index(_index_name: String, _language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> String {
	let mut _index_to_find: String = role_index(_index_name, _language, _client_id, _role_id);
	if let Some(user_id) = _user_id {
		if let Ok(id) = user_id.parse::<i32>() {
			if id >= 0 {
				_index_to_find.push_str("_");
				_index_to_find.push_str(user_id);
			}
		}
	}
	_index_to_find.to_lowercase()
}
