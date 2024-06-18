use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index}};

use super::role::Role;

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
    pub id: Option<i32>,
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
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
    // Supported References
    pub action: Option<String>,
    pub action_id: Option<i32>,
    pub action_uuid: Option<String>,
    pub window: Option<Window>,
    pub process: Option<Process>,
    pub form: Option<Form>,
	pub browser: Option<Browser>,
    pub workflow: Option<Workflow>,
    // Tree menu childs
    pub children: Option<Vec<MenuItem>>
}

impl Default for MenuItem {
    fn default() -> Self {
        Self { 
            uuid: None, 
            id: None, 
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
			// Tree menu childs
			children: None
        }
    }
}

impl MenuItem {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut menu = MenuItem::default();
        menu.id = _id;
        menu
    }

    fn get_find_body_from_role(self: &Self, _role: Role) -> serde_json::Value {
        // "W" Window
        // "X" Form
        // "S" Smart Browser
        // "R" Report
        // "P" Process
        // "F" Workflow
        let _window_access = match _role.to_owned().window_access {
            Some(value) => value,
            None => Vec::new()
        };
        let _form_access = match _role.to_owned().form_access {
            Some(value) => value,
            None => Vec::new()
        };
        let _browser_access = match _role.to_owned().browser_access {
            Some(value) => value,
            None => Vec::new()
        };
        let _process_access = match _role.to_owned().process_access {
            Some(value) => value,
            None => Vec::new()
        };
        let _workflow_access = match _role.to_owned().workflow_access {
            Some(value) => value,
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
                          "terms": {
                            "action_id": _window_access
                          }
                        },
                        {
                          "match": {
                            "action": "W"
                          }
                        }
                      ]
                    }
                  },
                  {
                    "bool": {
                      "must": [
                        {
                          "terms": {
                            "action_id": _form_access
                          }
                        },
                        {
                          "match": {
                            "action": "X"
                          }
                        }
                      ]
                    }
                  },
                  {
                    "bool": {
                      "must": [
                        {
                          "terms": {
                            "action_id": _browser_access
                          }
                        },
                        {
                          "match": {
                            "action": "S"
                          }
                        }
                      ]
                    }
                  },
                  {
                    "bool": {
                      "must": [
                        {
                          "terms": {
                            "action_id": _process_access
                          }
                        },
                        {
                          "terms": {
                            "action": ["R", "P"]
                          }
                        }
                      ]
                    }
                  },
                  {
                    "bool": {
                      "must": [
                        {
                          "terms": {
                            "action_id": _workflow_access
                          }
                        },
                        {
                          "match": {
                            "action": "F"
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
}

impl IndexDocument for MenuItem {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "integer" },
                    "parent_id" : { "type" : "integer" },
                    "sequence" : { "type" : "integer" },
                    "name" : { "type" : "text" },
                    "description" : { "type" : "text" }
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
            None => "menu".to_string(),
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
pub struct Window {
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
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Form {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browser {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Workflow {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

pub async fn menu_from_id(_id: Option<i32>, _language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> Result<MenuItem, String> {
	if _id.is_none() || _id.map(|id| id <= 0).unwrap_or(false) {
		return Err(Error::new(ErrorKind::InvalidData.into(), "MenuItem Identifier is Mandatory").to_string());
	}
    let mut _document = MenuItem::from_id(_id);

	let _index_name = match get_index_name(_language, _client_id, _role_id, _user_id).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(error.to_string())
		}
	};
	log::info!("Index to search {:}", _index_name);

	_document.index_value = Some(_index_name);
    let _menu_document: &dyn IndexDocument = &_document;
    match get_by_id(_menu_document).await {
        Ok(value) => {
			let mut menu: MenuItem = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", menu.id);

			// sort menu children nodes by sequence
			if let Some(ref mut children) = menu.children {
				children.sort_by_key(|child| child.sequence.clone().unwrap_or(0));
			}

            Ok(
                menu
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

	let _index: String = "menu".to_string();

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

pub async fn menus(
	_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>,
	_search_value: Option<&String>, _page_number: Option<&String>, _page_size: Option<&String>
) -> Result<MenuItemListResponse, std::io::Error> {
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

    let mut _document = MenuItem::default();
    _document.index_value = Some(_index_name);
    let _menu_document: &dyn IndexDocument = &_document;

	// pagination
	let page_number: i64 = match _page_number {
		Some(value) => value.clone().parse::<i64>().to_owned(),
		None => "0".parse::<i64>().to_owned()
	}.unwrap();
	let page_size: i64 = match _page_size {
		Some(value) => value.clone().parse::<i64>().to_owned(),
		None => "100".parse::<i64>().to_owned()
	}.unwrap();

    match find(_menu_document, _search_value, page_number, page_size).await {
        Ok(values) => {
            let mut menus_list: Vec<MenuItem> = vec![];
            for value in values {
				let mut menu: MenuItem = serde_json::from_value(value).unwrap();
				// sort menu children nodes by sequence
				if let Some(ref mut children) = menu.children {
					children.sort_by_key(|child| child.sequence.clone().unwrap_or(0));
				}
                menus_list.push(menu.to_owned());
            }

			// sort root menu nodes by sequence
			menus_list.sort_by_key(|menu| menu.sequence.clone().unwrap_or(0));

            Ok(MenuItemListResponse {
                menus: Some(menus_list)
            })
        },
		Err(error) => {
			Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
    }
}
