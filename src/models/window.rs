use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::{json, Value};
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{find, get_by_id, IndexDocument}, models::{get_index_name, role::{role_from_id, Role}}};

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
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
	pub is_beta_functionality: Option<bool>,
    pub window_type: Option<String>,
	pub is_sales_transaction: Option<bool>,
	//	Index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
	//	Tabs
    pub tabs: Option<Vec<WindowTab>>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct WindowTab {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
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
	pub filter_column_name: Option<String>,
	// External info
	pub context_column_names: Option<Vec<String>>,
	pub window_id: Option<i32>,
	pub process_id: Option<i32>,
	pub process_uuid: Option<String>,
	pub process: Option<Process>,
	pub processes_uuid: Option<Vec<String>>,
	pub processes: Option<Vec<Process>>,
	//	Fields
    pub fields: Option<Vec<WindowField>>
    // pub row_fields: Option<Vec<WindowField>>,
    // pub grid_fields: Option<Vec<WindowField>>
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

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Reference {
	pub table_name: Option<String>,
	pub reference_id: Option<i32>,
	pub reference_value_id: Option<i32>,
	pub context_column_names: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct WindowField {
	pub uuid: Option<String>,
	pub id: Option<String>,
	pub internal_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_active: Option<bool>,
	//
	pub display_type: Option<i32>,
	pub is_allow_copy: Option<bool>,
	pub is_heading: Option<bool>,
	pub is_field_only: Option<bool>,
	pub is_quick_entry: Option<bool>,
	//	Column Properties
    pub column_name: Option<String>,
	pub column_sql: Option<String>,
	pub is_key: Option<bool>,
	pub is_parent: Option<bool>,
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
    pub is_displayed_as_panel: Option<String>,
    pub is_displayed_as_table: Option<String>,
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
			internal_id: None,
            name: None, 
            description: None, 
            help: None, 
			is_active: None,
			is_beta_functionality: None,
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
    pub fn from_id(_id: Option<String>) -> Self {
		let mut window: Window = Window::default();
        window.id = _id;
        window
    }

	pub fn to_string(&self) -> String {
		format!("Window: UUID {:?}, ID {:?}, Name {:?}, Index: {:?}", self.uuid, self.internal_id, self.name, self.index_value)
	}
}

impl IndexDocument for Window {
	fn mapping(self: &Self) -> serde_json::Value {
		json!({
			"mappings" : {
				"properties" : {
					"uuid" : { "type" : "keyword" },
					"id" : { "type" : "keyword" },
					"internal_id" : { "type" : "integer" },
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
            None => "window".to_string(),
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
pub struct DictionaryEntity {
	pub uuid: Option<String>,
	pub internal_id: Option<i32>,
    pub id: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Process {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
	pub is_report: Option<bool>,
	pub is_multi_selection: Option<bool>,
	//	Linked
	pub browser_id: Option<i32>,
	pub browser: Option<DictionaryEntity>,
	pub form_id: Option<i32>,
	pub form: Option<DictionaryEntity>,
	pub workflow_id: Option<i32>,
	pub workflow: Option<DictionaryEntity>,
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


pub fn parse_window(value: Value, _process_access: Vec<String>) -> Window {
	let mut window: Window = serde_json::from_value(value).unwrap();

	// sort tabs list by sequence
	if let Some(ref mut tabs) = window.tabs {
		// sort tabs list by sequence
		tabs.sort_by_key(|tab: &WindowTab| tab.sequence.clone().unwrap_or(0));
		for tab in tabs.iter_mut() {
			// verify direct process access
			if let Some(ref mut process) = tab.process {
				if let Some(ref uuid) = process.uuid {
					if !_process_access.contains(uuid) {
						// set None if is without access
						tab.process = None;
					}
				} else {
					// set None if uuid is empty
					tab.process = None;
				}
			}

			// filter tab processes access
			if let Some(ref mut processes) = tab.processes {
				let filtered_processes: Vec<Process> = processes
					.iter()
					.filter(|process| {
						if let Some(ref uuid) = process.uuid {
							_process_access.contains(uuid)
						} else {
							false
						}
					})
					.cloned()
					.collect() // Especificamos el tipo de retorno aquí
				;
				// assing filter new access process on tab.processes
				*processes = filtered_processes;
			}

			// sort processes list by name
			if let Some(ref mut processes) = tab.processes {
				processes.sort_by_key(|process: &Process| process.name.clone().unwrap_or("".to_owned()));
			}
			// sort fields list by sequence
			if let Some(ref mut fields) = tab.fields {
				fields.sort_by_key(|field: &WindowField| field.sequence.clone().unwrap_or(0));
			}
		}
	}

	window.to_owned()
}


pub async fn window_from_id(_id: Option<String>, _language: Option<&String>, _dictionary_code: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>) -> Result<Window, String> {
	if _id.is_none() || _id.as_deref().map_or(false, |s| s.trim().is_empty()) {
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Window Identifier is Mandatory").to_string()
		);
	}
	let mut _document: Window = Window::from_id(_id.to_owned());

	let _index_name: String = match get_index_name("window".to_string(), _language,_dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error to {:?}: {:?}", _id.to_owned(), error.to_string());
			return Err(error.to_string())
		}
	};
	log::info!("Index to search {:}", _index_name);

	// load role
	let _expected_role: Result<Role, String> = role_from_id(_role_id, _client_id, _dictionary_code).await;
	let _role: Role = match _expected_role {
		Ok(role) => role,
		Err(error) => {
			log::error!("{}", error);
			return Err(error.to_string())
		},
	};

	let _process_access: Vec<String> = match _role.to_owned().process_access {
		Some(value) => {
			// remove none values into vector
			value.into_iter().flatten().collect()
		},
		None => Vec::new()
	};

    _document.index_value = Some(_index_name);
    let _window_document: &dyn IndexDocument = &_document;
    match get_by_id(_window_document).await {
        Ok(value) => {
			let window: Window = parse_window(value, _process_access);
			log::info!("Finded Window {:?} Value: {:?}", window.name, window.id);

            Ok(window)
        },
        Err(error) => {
			log::error!("{}", error);
            Err(error)
        },
    }
}

pub async fn windows(_language: Option<&String>, _search_value: Option<&String>, _dictionary_code: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>) -> Result<WindowListResponse, std::io::Error> {
	let _search_value: String = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };

	//  Find index
	let _index_name: String = match get_index_name("window".to_string(), _language, _dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};
	log::info!("Index to search {:}", _index_name);

	// load role
	let _expected_role: Result<Role, String> = role_from_id(_role_id, _client_id, _dictionary_code).await;
	let _role = match _expected_role {
		Ok(role) => role,
		Err(error) => {
			log::error!("{:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};

	let mut _document: Window = Window::default();
    _document.index_value = Some(_index_name);
    let _window_document: &dyn IndexDocument = &_document;
    match find(_window_document, _search_value, 0, 10).await {
        Ok(values) => {
		
			let _process_access: Vec<String> = match _role.to_owned().process_access {
				Some(value) => {
					// remove none values into vector
					value.into_iter().flatten().collect()
				},
				None => Vec::new()
			};

            let mut windows_list: Vec<Window> = vec![];
            for value in values {
				let window: Window = parse_window(value, _process_access.clone());
                windows_list.push(window.to_owned());
            }

            Ok(WindowListResponse {
                windows: Some(windows_list)
            })
        },
		Err(error) => {
			log::error!("{}", error);
			Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
    }
}
