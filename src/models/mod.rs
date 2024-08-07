pub mod browser;
pub mod form;
pub mod menu;
pub mod process;
pub mod window;
pub mod generic;
pub mod menu_item;
pub mod menu_tree;
pub mod role;

use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use std::{io::ErrorKind, io::Error};
use crate::controller::opensearch::exists_index;

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
		if !language.trim().is_empty() {
			_index_to_find.push_str("_");
			_index_to_find.push_str(language);
		}
	}
	_index_to_find.to_lowercase()
}

fn client_index_only(_index_name: String, _client_id: Option<&String>) -> String {
	let mut _index_to_find: String = default_index(_index_name);
	if let Some(client) = _client_id {
		if !client.trim().is_empty() {
			_index_to_find.push_str("_");
			_index_to_find.push_str(client);
		}
	}
	_index_to_find.to_lowercase()
}

async fn get_index_name(_index_name: String, _language: Option<&String>, _dictionary_code: Option<&String>) -> Result<String, std::io::Error> {
	//  Validate
	if _language.is_none() || _language.as_deref().map_or(false, |s| s.trim().is_empty()) {
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Language is Mandatory")
		);
	}

	let mut _language_index = language_index(_index_name, _language);
	if let Some(code) = _dictionary_code {
		if !code.trim().is_empty() {
			_language_index.push_str("_");
			_language_index.push_str(code);
		}
	}

	//  Find index
	match exists_index(_language_index.to_owned()).await {
		Ok(_) => {
			log::info!("Find with language index `{:}`", _language_index);
			Ok(_language_index)
		},
		Err(error) => {
			log::warn!("No menu item index `{:}`", _language_index);
			log::error!("No role index `{:}`", _language_index);
			return Err(
				Error::new(ErrorKind::InvalidData.into(), error)
			)
		}
	}
}
