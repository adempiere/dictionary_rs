use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index, default_index, language_index, client_index}};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct WindowDocument {
    pub document: Option<Window>
}

#[derive(Serialize, Debug, Clone)]
pub struct WindowResponse {
    pub window: Option<Window>
}

#[derive(Serialize, Debug, Clone)]
pub struct WindowListResponse {
    pub windows: Option<Vec<Window>>
}

impl Default for WindowResponse {
    fn default() -> Self {
        WindowResponse { 
            window: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Window {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub window_type: Option<String>,
	pub is_sales_transaction: Option<bool>,
	//	Index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
	//	Tabs
    pub tabs: Option<Vec<WindowTab>>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct WindowTab {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	// Record attributes
	pub is_insert_record: Option<bool>,
    pub commit_warning: Option<String>,
	// Attributes
    pub display_logic: Option<String>,
	pub sequence: Option<i32>,
	pub tab_level: Option<i32>,
	pub is_read_only: Option<bool>,
    pub read_only_logic: Option<String>,
    pub is_single_row: Option<bool>,
    pub is_advanced_tab: Option<bool>,
	pub is_has_tree: Option<bool>,
    pub is_info_tab: Option<bool>,
    pub is_translation_tab: Option<bool>,
	// Table attributes
	pub table_name: Option<String>,
    pub table: Option<Table>,
	// Link attributes
	pub parent_column_name: Option<String>,
	pub link_column_name: Option<String>,
	// Sort attributes
	pub is_sort_tab: Option<bool>,
	pub sort_order_column_name: Option<String>,
	pub sort_yes_no_column_name: Option<String>,
	// External info
	pub context_columns: Option<Vec<String>>,
	pub window_id: Option<i32>,
	pub process_id: Option<i32>,
	pub process: Option<Process>,
	pub processes: Option<Vec<Process>>,
	//	Fields
    pub fields: Option<Vec<WindowField>>,
    pub row_fields: Option<Vec<WindowField>>,
    pub grid_fields: Option<Vec<WindowField>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct DependendField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub column_name: Option<String>,
    pub parent_id: Option<i32>,
    pub parent_uuid: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Reference {
	pub table_name: Option<String>,
	pub context_column_names: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct WindowField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	//
	pub display_type: Option<i32>,
	pub is_allow_copy: Option<bool>,
	pub is_heading: Option<bool>,
	pub is_field_only: Option<bool>,
	//	Column Properties
    pub column_name: Option<String>,
	pub column_sql: Option<String>,
	pub is_key: Option<bool>,
	pub is_translated: Option<bool>,
	pub is_identifier: Option<bool>,
	pub identifier_sequence: Option<i32>,
	pub is_selection_column: Option<bool>,
	pub callout: Option<String>,
	//	Value Properties
    pub default_value: Option<String>,
	pub field_length: Option<i32>,
	pub value_format: Option<String>,
	pub format_pattern: Option<String>,
	pub value_min: Option<String>,
	pub value_max: Option<String>,
	pub is_encrypted: Option<bool>,
	//	Display Properties
	pub is_displayed: Option<bool>,
    pub display_logic: Option<String>,
    pub sequence: Option<i32>,
	pub is_displayed_grid: Option<bool>,
    pub grid_sequence: Option<i32>,
	//	Editable Properties
    pub is_read_only: Option<bool>,
	pub read_only_logic: Option<String>,
	pub is_updateable: Option<bool>,
	pub is_always_updateable: Option<bool>,
	//	Mandatory Properties
	pub is_mandatory: Option<bool>,
	pub mandatory_logic: Option<String>,
	//	External Info
    pub context_column_names: Option<Vec<String>>,
	pub reference: Option<Reference>,
	pub dependent_fields: Option<Vec<DependendField>>,
	pub process_id: Option<i32>,
	pub process: Option<Process>
}

impl Default for Window {
    fn default() -> Self {
        Self { 
            uuid: None, 
            id: None, 
            name: None, 
            description: None, 
            help: None, 
            client_id: None,
            index_value: None,
            language: None,
            role_id: None,
            user_id: None,
            is_sales_transaction: None,
            tabs: None,
            window_type: None
        }
    }
}

impl Window {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut window = Window::default();
        window.id = _id;
        window
    }
}

impl IndexDocument for Window {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "integer" },
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
            None => "window".to_string(),
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
pub struct DictionaryEntity {
	pub uuid: Option<String>,
	pub id: Option<i32>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Process {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_report: Option<bool>,
	//	Linked
	pub browser_id: Option<i32>,
	pub browser: Option<DictionaryEntity>,
	pub from_id: Option<i32>,
	pub form: Option<DictionaryEntity>,
	pub workflow_id: Option<i32>,
	pub workflow: Option<DictionaryEntity>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Table {
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

pub async fn window_from_id(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _id: Option<i32>) -> Result<Window, String> {
    let mut _document = Window::from_id(_id);
    let _index_name = get_index_name(_language, _client_id, _role_id, _user_id).await.expect("Error getting index");
    log::info!("Index to search {:}", _index_name);
    _document.index_value = Some(_index_name);
    let _window_document: &dyn IndexDocument = &_document;
    match get_by_id(_window_document).await {
        Ok(value) => {
            let window: Window = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", window.id);
            // Ok(WindowResponse {
            //     window: Some(window)
            // })
            Ok(window)
        },
        Err(error) => {
            log::warn!("{}", error);
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
    let _index = "window".to_string();
    let _user_index = match _user_id {
        Some(_) => user_index(_index.to_owned(), _language, _client_id, _role_id, _user_id),
        None => role_index(_index.to_owned(), _language, _client_id, _role_id)
    };
    let _role_index = role_index(_index.to_owned(), _language, _client_id, _role_id);
    let _client_index = client_index(_index.to_owned(), _language, _client_id, _role_id);
    let _language_index = language_index(_index.to_owned(), _language, _client_id, _role_id);
    let _default_index = default_index(_index.to_owned(), _language, _client_id, _role_id);
    //  Find index
    match exists_index(_user_index.to_owned()).await {
        Ok(_) => Ok(_user_index),
        Err(_) => {
            match exists_index(_role_index.to_owned()).await {
                Ok(_) => Ok(_role_index),
                Err(_) => {
                    match exists_index(_client_index.to_owned()).await {
                        Ok(_) => Ok(_client_index),
                        Err(_) => {
                            match exists_index(_language_index.to_owned()).await {
                                Ok(_) => Ok(_language_index),
                                Err(_) => {
                                    Ok(_default_index)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn windows(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _search_value: Option<&String>) -> Result<WindowListResponse, std::io::Error> {
    let _search_value = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };
    //  Find index
    let _index_name = get_index_name(_language, _client_id, _role_id, _user_id).await.expect("Error getting index");
    log::info!("Index to search {:}", _index_name);
    let mut _document = Window::default();
    _document.index_value = Some(_index_name);
    let _window_document: &dyn IndexDocument = &_document;
    match find(_window_document, _search_value, 0, 10).await {
        Ok(values) => {
            let mut windows_list: Vec<Window> = vec![];
            for value in values {
                let window: Window = serde_json::from_value(value).unwrap();
                windows_list.push(window.to_owned());
            }
            Ok(WindowListResponse {
                windows: Some(windows_list)
            })
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
    // Ok(WindowResponse::default())
}