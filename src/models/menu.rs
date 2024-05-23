use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index, client_index, language_index, default_index}};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct MenuDocument {
    pub document: Option<Menu>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuResponse {
    pub menu: Option<Menu>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuListResponse {
    pub menus: Option<Vec<Menu>>
}

impl Default for MenuResponse {
    fn default() -> Self {
        MenuResponse { 
            menu: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Menu {
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
    pub browse: Option<Browse>,
    pub workflow: Option<Workflow>,
    // Tree menu childs
    pub children: Option<Vec<Menu>>
}

impl Default for Menu {
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
            browse: None,
			workflow: None,
			// Tree menu childs
			children: None
        }
    }
}

impl Menu {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut menu = Menu::default();
        menu.id = _id;
        menu
    }
}

impl IndexDocument for Menu {
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
pub struct Browse {
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

pub async fn menu_from_id(_id: Option<i32>) -> Result<Menu, String> {
    let mut _document = Menu::from_id(_id);
    let _menu_document: &dyn IndexDocument = &_document;
    match get_by_id(_menu_document).await {
        Ok(value) => {
            let menu: Menu = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", menu.id);
            // Ok(MenuResponse {
            //     menu: Some(menu)
            // })
            Ok(
                menu
            )
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

	let _index: String = "menu".to_string();

	let _user_index = user_index(_index.to_owned(), _language, _client_id, _role_id, _user_id);
    let _role_index = role_index(_index.to_owned(), _language, _client_id, _role_id);
	let _client_index = client_index(_index.to_owned(), _language, _client_id);
	let _language_index = language_index(_index.to_owned(), _language);
	let _default_index = default_index(_index.to_owned());

	//  Find index
	match exists_index(_user_index.to_owned()).await {
		Ok(_) => {
			log::info!("Find with user index `{:}`", _user_index);
			Ok(_user_index)
		},
		Err(_) => {
			log::info!("No user index `{:}`", _user_index);
			match exists_index(_role_index.to_owned()).await {
				Ok(_) => {
					log::info!("Find with role index `{:}`", _role_index);
					Ok(_role_index)
				},
				Err(_) => {
					log::info!("No role index `{:}`", _role_index);
					match exists_index(_client_index.to_owned()).await {
						Ok(_) => {
							log::info!("Find with client index `{:}`", _client_index);
							Ok(_client_index)
						},
						Err(_) => {
							log::info!("No client index `{:}`", _client_index);
							match exists_index(_language_index.to_owned()).await {
								Ok(_) => {
									log::info!("Find with language index `{:}`", _language_index);
									Ok(_language_index)
								},
								Err(_) => {
									log::info!("No language index `{:}`. Find with default index `{:}`.", _language_index, _default_index);
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

pub async fn menus(
	_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>,
	_search_value: Option<&String>, _page_number: Option<&String>, _page_size: Option<&String>
) -> Result<MenuListResponse, std::io::Error> {
	let _search_value = match _search_value {
		Some(value) => value.clone(),
		None => "".to_owned()
	};

	let _index_name = get_index_name(_language, _client_id, _role_id, _user_id).await.expect("Error getting index");
    log::info!("Index to search {:}", _index_name);

    let mut _document = Menu::default();
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
            let mut menus_list: Vec<Menu> = vec![];
            for value in values {
                let menu: Menu = serde_json::from_value(value).unwrap();
                menus_list.push(menu.to_owned());
            }
            Ok(MenuListResponse {
                menus: Some(menus_list)
            })
        },
		Err(error) => {
			Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
    }
    // Ok(MenuResponse::default())
}
