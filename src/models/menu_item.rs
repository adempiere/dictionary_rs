use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::controller::opensearch::{exists_index, find_from_dsl_body, IndexDocument};

use super::{language_index, menu::{Browser, Form, Process, Window, Workflow}, role::Role};

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
    pub workflow: Option<Workflow>
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
            workflow: None
        }
    }
}

impl MenuItem {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut menu = MenuItem::default();
        menu.id = _id;
        menu
    }

    fn get_find_body_from_role(_role: Role) -> serde_json::Value {
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

pub async fn menu_items_from_role(_role: Role, _language: Option<&String>, _page_number: Option<i64>, _page_size: Option<i64>) -> Result<Vec<MenuItem>, std::io::Error> {
	let mut _search_body = MenuItem::get_find_body_from_role(_role);
	let _index_name = match get_index_name(_language).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	};
	// log::info!("Index to search {:}", _index_name);
  // pagination
  let page_number: i64 = match _page_number {
    Some(value) => value,
    None => 0
  };
  let page_size: i64 = match _page_size {
    Some(value) => value,
    None => 100
  };

  match find_from_dsl_body("menu_item_es_mx".to_string(), _search_body, page_number, page_size).await {
    Ok(values) => Ok(values.iter().map(|_value| serde_json::from_value(_value.clone()).unwrap()).collect::<Vec<MenuItem>>()),
    Err(error) => {
      Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
  }
}
async fn get_index_name(_language: Option<&String>) -> Result<String, std::io::Error> {
	//  Validate
	if _language.is_none() {
		return Err(Error::new(ErrorKind::InvalidData.into(), "Language is Mandatory"));
	}
	let _index: String = "menu_item".to_string();
	let _language_index = language_index(_index.to_owned(), _language);
	//  Find index
	match exists_index(_language_index.to_owned()).await {
		Ok(_) => {
			log::info!("Find with language index `{:}`", _language_index);
			Ok(_language_index)
		},
		Err(error) => {
			log::warn!("No menu item index `{:}`", _language_index);
			log::error!("No role index `{:}`", _language_index);
        return Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
	}
}
