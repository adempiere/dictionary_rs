use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::{json, Value};
use std::{io::ErrorKind, io::Error};

use crate::controller::opensearch::{find_from_dsl_body, IndexDocument};

use super::{get_index_name, menu::MenuAction, role::Role};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct MenuItemDocument {
    pub document: Option<MenuItem>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuItemResponse {
    pub menu: Option<MenuItem>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuItemListResponse {
    pub menus: Option<Vec<MenuItem>>
}

impl Default for MenuItemResponse {
    fn default() -> Self {
        MenuItemResponse { 
            menu: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct MenuItem {
	pub uuid: Option<String>,
	pub id: Option<String>,
	pub internal_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub sequence: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_summary: Option<bool>,
    pub is_sales_transaction: Option<bool>,
    pub is_read_only: Option<bool>,
    // index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    // Supported References
	pub action: Option<String>,
	pub action_id: Option<i32>,
	pub action_uuid: Option<String>,
	pub window: Option<MenuAction>,
	pub process: Option<MenuAction>,
	pub form: Option<MenuAction>,
	pub browser: Option<MenuAction>,
	pub workflow: Option<MenuAction>,
	// New UI
	pub web_path: Option<String>,
	pub module_id: Option<i32>,
	pub sub_module_id: Option<i32>
}

impl Default for MenuItem {
	fn default() -> Self {
		Self {
			uuid: None,
			id: None,
			internal_id: None,
            parent_id: None, 
            sequence: None, 
            name: None, 
            description: None, 
            is_summary: None, 
            is_sales_transaction: None, 
            is_read_only: None, 
            // index
            index_value: None,
            language: None,
            client_id: None,
            role_id: None,
            user_id: None,
            // Supported References
			action: None,
			action_id: None,
			action_uuid: None,
			window: None,
			process: None,
			form: None,
			browser: None,
			workflow: None,
			// New UI
			web_path: None,
			module_id: None,
			sub_module_id: None
        }
    }
}

impl MenuItem {
	pub fn from_id(_id: Option<String>) -> Self {
		let mut menu: MenuItem = MenuItem::default();
        menu.id = _id;
        menu
    }

    fn get_find_body_from_role(_role: Role) -> serde_json::Value {
		// "W" Window
		let _window_access: Vec<String> = match _role.to_owned().window_access {
			Some(value) => {
				// remove none values into vector
				value.into_iter().flatten().collect()
			},
			None => Vec::new()
		};

		// "X" Form
		let _form_access: Vec<String> = match _role.to_owned().form_access {
			Some(value) => {
				// remove none values into vector
				value.into_iter().flatten().collect()
			},
			None => Vec::new()
		};

		// "S" Smart Browser
		let _browser_access: Vec<String> = match _role.to_owned().browser_access {
			Some(value) => {
				// remove none values into vector
				value.into_iter().flatten().collect()
			},
			None => Vec::new()
		};

		// "R" Report
		// "P" Process
		let _process_access: Vec<String> = match _role.to_owned().process_access {
			Some(value) => {
				// remove none values into vector
				value.into_iter().flatten().collect()
			},
			None => Vec::new()
		};

		// "F" Workflow
		let _workflow_access: Vec<String> = match _role.to_owned().workflow_access {
			Some(value) => {
				// remove none values into vector
				value.into_iter().flatten().collect()
			},
			None => Vec::new()
		};

		json!({
			"query": {
				"bool": {
					"should": [
						{
							"bool": {
								"must": [
									{
										"match": {
											"is_summary": true
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "W"
										}
									},
									{
										"terms": {
											"action_uuid": _window_access
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "X"
										}
									},
									{
										"terms": {
											"action_uuid": _form_access
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "S"
										}
									},
									{
										"terms": {
											"action_uuid": _browser_access
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "P"
										}
									},
									{
										"terms": {
											"action_uuid": _process_access
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "R"
										}
									},
									{
										"terms": {
											"action_uuid": _process_access
										}
									}
								]
							}
						},
						{
							"bool": {
								"must": [
									{
										"match": {
											"action": "F"
										}
									},
									{
										"terms": {
											"action_uuid": _workflow_access
										}
									}
								]
							}
						}
					]
				}
			}
		})
	}

	pub fn to_string(&self) -> String {
		format!("Menu Item: UUID {:?}, ID {:?}, Name {:?}, Index: {:?}", self.uuid, self.internal_id, self.name, self.index_value)
	}
}

impl IndexDocument for MenuItem {
	fn mapping(self: &Self) -> serde_json::Value {
		json!({
			"mappings" : {
				"properties" : {
					"uuid" : { "type" : "keyword" },
					"id" : { "type" : "keyword" },
					"internal_id" : { "type" : "integer" },
                    "parent_id" : { "type" : "integer" },
                    "sequence" : { "type" : "integer" },
                    "name" : { "type" : "text" },
					"description" : { "type" : "text" },
					"action_id" : { "type" : "integer" },
					"action_uuid" : { "type" : "keyword" }
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
            None => "menu".to_string(),
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

pub async fn menu_items_from_role(_role: Role, _language: Option<&String>, _dictionary_code: Option<&String>, _page_number: Option<i64>, _page_size: Option<i64>) -> Result<Vec<MenuItem>, std::io::Error> {
	let mut _search_body: Value = MenuItem::get_find_body_from_role(_role);
	let _index_name: String = match get_index_name("menu_item".to_string(), _language,_dictionary_code).await {
		Ok(index_name) => index_name,
		Err(error) => {
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};

  // pagination
  let page_number: i64 = match _page_number {
    Some(value) => value,
    None => 0
  };
  let page_size: i64 = match _page_size {
    Some(value) => value,
    None => 10000
  };

  match find_from_dsl_body(_index_name, _search_body, page_number, page_size).await {
	Ok(values) => {
		Ok(values.iter().map(|_value| serde_json::from_value(_value.clone()).unwrap()).collect::<Vec<MenuItem>>())
	},
    Err(error) => {
      Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
  }
}
