use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index}};

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
	pub id: Option<i32>,
	pub uuid: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browser {
    pub uuid: Option<String>,
    pub id: Option<i32>,
	pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
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
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
	// External Reference
	pub context_column_names: Option<Vec<String>>,
	pub process_id: Option<i32>,
	pub process: Option<DictionaryEntity>,
	pub window_id: Option<i32>,
	pub window: Option<DictionaryEntity>,
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
	pub context_column_names: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct BrowserField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
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
			code: None,
            name: None, 
            description: None, 
            help: None, 
			is_active: None,
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
			process: None,
			window_id: None,
			window: None,
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
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut browser = Browser::default();
        browser.id = _id;
        browser
    }
}

impl IndexDocument for Browser {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "integer" },
                    "code" : { "type" : "text" },
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
        self.id.unwrap().to_string()
    }

    fn index_name(self: &Self) -> String {
        match &self.index_value {
            Some(value) => value.to_string(),
            None => "browser".to_string(),
        }
    }

    fn find(self: &Self, _search_value: String) -> serde_json::Value {
        let mut query = "*".to_owned();
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
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Window {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Table {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub table_name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub is_document: Option<bool>,
    pub is_deleteable: Option<bool>,
    pub is_view: Option<bool>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct DependendField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub column_name: Option<String>,
    pub parent_id: Option<i32>,
    pub parent_uuid: Option<String>,
    pub parent_name: Option<String>
}

pub async fn browser_from_id(_id: Option<i32>, _language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> Result<Browser, String> {
	if _id.is_none() || _id.map(|id| id <= 0).unwrap_or(false) {
		return Err(Error::new(ErrorKind::InvalidData.into(), "Browser Identifier is Mandatory").to_string());
	}
    let mut _document = Browser::from_id(_id);

	let _index_name = match get_index_name(_language, _client_id, _role_id, _user_id).await {
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
            let browser: Browser = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", browser.id);
            // Ok(BrowserResponse {
            //     browser: Some(browser)
            // })
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

async fn get_index_name(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> Result<String, std::io::Error> {
    //  Validate
    if _language.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Language is Mandatory"));
    }
    if _client_id.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Client is Mandatory"));
    }
    if _role_id.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Role is Mandatory"));
    }

	let _index: String = "browser".to_string();

	let _user_index = user_index(_index.to_owned(), _language, _client_id, _role_id, _user_id);
    let _role_index = role_index(_index.to_owned(), _language, _client_id, _role_id);

	//  Find index
    match exists_index(_user_index.to_owned()).await {
		Ok(_) => {
			log::info!("Find with user index `{:}`", _user_index);
			Ok(_user_index)
		},
        Err(_) => {
			log::warn!("No user index `{:}`", _user_index);
            match exists_index(_role_index.to_owned()).await {
                Ok(_) => {
					log::info!("Find with role index `{:}`", _role_index);
					Ok(_role_index)
				},
				Err(error) => {
					log::error!("No role index `{:}`", _role_index);
					return Err(Error::new(ErrorKind::InvalidData.into(), error))
                }
            }
        }
    }
}

pub async fn browsers(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _search_value: Option<&String>) -> Result<BrowserListResponse, std::io::Error> {
    let _search_value = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };

	//  Find index
	let _index_name = match get_index_name(_language, _client_id, _role_id, _user_id).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};
	log::info!("Index to search {:}", _index_name);

    let mut _document = Browser::default();
    _document.index_value = Some(_index_name);
    let _browser_document: &dyn IndexDocument = &_document;
    match find(_browser_document, _search_value, 0, 10).await {
        Ok(values) => {
            let mut browsers_list: Vec<Browser> = vec![];
            for value in values {
                let browser: Browser = serde_json::from_value(value).unwrap();
                browsers_list.push(browser.to_owned());
            }
            Ok(BrowserListResponse {
                browsers: Some(browsers_list)
            })
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
}