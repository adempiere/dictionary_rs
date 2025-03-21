use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::{json, Value};
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{find, get_by_id, IndexDocument}, models::get_index_name};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct BrowserDocument {
    pub document: Option<Browser>
}

#[derive(Serialize, Debug, Clone)]
pub struct BrowserResponse {
    pub browser: Option<Browser>
}

#[derive(Serialize, Debug, Clone)]
pub struct BrowserListResponse {
    pub browsers: Option<Vec<Browser>>
}

impl Default for BrowserResponse {
    fn default() -> Self {
        BrowserResponse { 
            browser: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct DictionaryEntity {
	pub internal_id: Option<i32>,
    pub id: Option<String>,
	pub uuid: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browser {
	pub uuid: Option<String>,
	pub internal_id: Option<i32>,
	pub id: Option<String>,
	pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
	pub is_beta_functionality: Option<bool>,
	pub is_execute_query_by_default: Option<bool>,
	pub is_collapsible_by_default: Option<bool>,
	pub is_selected_by_default: Option<bool>,
	pub is_show_total: Option<bool>,
	pub field_key: Option<String>,
	// Record Attributes
    pub access_level: Option<String>,
	pub is_updateable: Option<bool>,
	pub is_deleteable: Option<bool>,
	pub table_name: Option<String>,
	pub table: Option<Table>,
	//	Index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
	// External Reference
	pub context_column_names: Option<Vec<String>>,
	pub process_id: Option<i32>,
	pub process_uuid: Option<String>,
	pub process: Option<DictionaryEntity>,
	pub window_id: Option<i32>,
	pub window: Option<DictionaryEntity>,
	pub is_search_process: Option<bool>,
	//	Browse Fields
	pub fields: Option<Vec<BrowserField>>
	// pub display_fields: Option<Vec<BrowserField>>,
	// pub criteria_fields: Option<Vec<BrowserField>>,
	// pub identifier_fields: Option<Vec<BrowserField>>,
	// pub order_fields: Option<Vec<BrowserField>>,
	// pub editable_fields: Option<Vec<BrowserField>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Reference {
	pub table_name: Option<String>,
	pub reference_id: Option<i32>,
	pub reference_value_id: Option<i32>,
	pub context_column_names: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct BrowserField {
	pub uuid: Option<String>,
	pub id: Option<String>,
	pub internal_id: Option<i32>,
	pub column_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
	pub display_type: Option<i32>,
	pub callout: Option<String>,
	pub is_order_by: Option<bool>,
	pub sort_sequence: Option<i32>,
	pub is_key: Option<bool>,
	pub is_identifier: Option<bool>,
	//	Value Properties
    pub is_range: Option<bool>,
    pub default_value: Option<String>,
    pub default_value_to: Option<String>,
    pub value_format: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
	//	Display Properties
    pub is_displayed: Option<bool>,
    pub is_query_criteria: Option<bool>,
	pub display_logic: Option<String>,
	pub sequence: Option<i32>,
	pub grid_sequence: Option<i32>,
    pub is_displayed_as_panel: Option<String>,
    pub is_displayed_as_table: Option<String>,
	//	Editable Properties
    pub is_read_only: Option<bool>,
	pub read_only_logic: Option<String>,
	pub is_info_only: Option<bool>,
	//	Mandatory Properties
	pub is_mandatory: Option<bool>,
	//	External Info
	pub element_name: Option<String>,
    pub context_column_names: Option<Vec<String>>,
	pub reference: Option<Reference>,
    pub dependent_fields: Option<Vec<DependendField>>
}

impl Default for Browser {
	fn default() -> Self {
		Self {
			uuid: None,
			id: None,
			internal_id: None,
			code: None,
            name: None, 
            description: None, 
            help: None, 
			is_active: None,
			is_beta_functionality: None,
            is_execute_query_by_default: None,
			is_collapsible_by_default: None,
            is_selected_by_default: None,
            is_show_total: None,
			field_key: None,
			// Record Attributes
			access_level: None,
            is_updateable: None,
			is_deleteable: None,
			table_name: None,
            table: None,
			//	Index
            index_value: None,
            language: None,
			client_id: None,
            role_id: None,
			user_id: None,
			// External Reference
			context_column_names: None,
			process_id: None,
			process_uuid: None,
			process: None,
			window_id: None,
			window: None,
			is_search_process: None,
			//	Browse Fields
			fields: None
			// display_fields: None,
			// criteria_fields: None,
			// identifier_fields: None,
			// order_fields: None,
			// editable_fields: None
		}
	}
}

impl Browser {
    pub fn from_id(_id: Option<String>) -> Self {
		let mut browser: Browser = Browser::default();
        browser.id = _id;
        browser
    }

	pub fn to_string(&self) -> String {
		format!("Browser: UUID {:?}, ID {:?}, Name {:?}, Index: {:?}", self.uuid, self.internal_id, self.name, self.index_value)
	}
}

impl IndexDocument for Browser {
	fn mapping(self: &Self) -> serde_json::Value {
		json!({
			"mappings" : {
				"properties" : {
					"uuid" : { "type" : "keyword" },
					"id" : { "type" : "keyword" },
					"internal_id" : { "type" : "integer" },
					"code" : { "type" : "keyword" },
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
            None => "browser".to_string(),
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

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Process {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Window {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Table {
	pub uuid: Option<String>,
	pub internal_id: Option<i32>,
	pub id: Option<String>,
	pub table_name: Option<String>,
	pub access_level: Option<String>,
	pub key_columns: Option<Vec<String>>,
	pub is_view: Option<bool>,
	pub is_document: Option<bool>,
	pub is_deleteable: Option<bool>,
	pub is_change_log: Option<bool>,
	pub identifier_columns: Option<Vec<String>>,
	pub selection_colums: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct DependendField {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub column_name: Option<String>,
    pub parent_id: Option<i32>,
    pub parent_uuid: Option<String>,
    pub parent_name: Option<String>
}


pub fn parse_browser(value: Value) -> Browser {
	let mut browser: Browser = serde_json::from_value(value).unwrap();

	// sort fields by sequence
	if let Some(ref mut fields) = browser.fields {
		fields.sort_by_key(|field| field.sequence.clone().unwrap_or(0));
	}
	browser.to_owned()
}


pub async fn browser_from_id(_id: Option<String>, _language: Option<&String>, _dictionary_code: Option<&String>) -> Result<Browser, String> {
	if _id.is_none() || _id.as_deref().map_or(false, |s| s.trim().is_empty()) {
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Browser Identifier is Mandatory").to_string()
		);
	}
	let mut _document: Browser = Browser::from_id(_id);

	let _index_name: String = match get_index_name("browser".to_string(),_language, _dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(error.to_string())
		}
	};
	log::info!("Index to search {:}", _index_name);

    _document.index_value = Some(_index_name);
    let _browser_document: &dyn IndexDocument = &_document;
    match get_by_id(_browser_document).await {
        Ok(value) => {
			let browser: Browser = parse_browser(value);
			log::info!("Finded Browser {:?}: {:?}", browser.name, browser.id);

			Ok(
				browser
			)
        },
        Err(error) => {
			log::error!("{}", error);
            Err(error)
        },
    }
}

pub async fn browsers(_language: Option<&String>, _search_value: Option<&String>, _dictionary_code: Option<&String>) -> Result<BrowserListResponse, std::io::Error> {
	let _search_value: String = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };

	//  Find index
	let _index_name: String = match get_index_name("browser".to_string(), _language, _dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};
	log::info!("Index to search {:}", _index_name);

	let mut _document: Browser = Browser::default();
    _document.index_value = Some(_index_name);
    let _browser_document: &dyn IndexDocument = &_document;
    match find(_browser_document, _search_value, 0, 10).await {
        Ok(values) => {
			let mut browsers_list: Vec<Browser> = vec![];
            for value in values {
				let browser: Browser = parse_browser(value);

                browsers_list.push(browser.to_owned());
            }
            Ok(BrowserListResponse {
                browsers: Some(browsers_list)
            })
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
}
