use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index}};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct RoleDocument {
    pub document: Option<Role>
}

#[derive(Serialize, Debug, Clone)]
pub struct RoleResponse {
    pub role: Option<Role>
}

#[derive(Serialize, Debug, Clone)]
pub struct RoleListResponse {
    pub roles: Option<Vec<Role>>
}

impl Default for RoleResponse {
    fn default() -> Self {
        RoleResponse { 
            role: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Role {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tree_id: Option<i32>,
    // index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
    // Access
    pub window_access: Option<Vec<i32>>,
    pub process_access: Option<Vec<i32>>,
    pub form_access: Option<Vec<i32>>,
    pub browser_access: Option<Vec<i32>>,
    pub workflow_access: Option<Vec<i32>>,
    pub dashboard_access: Option<Vec<i32>>
}

impl Default for Role {
    fn default() -> Self {
        Self { 
            id: None, 
            uuid: None, 
            name: None, 
            description: None, 
            tree_id: None, 
            // index
			index_value: None,
			language: None,
			client_id: None,
			role_id: None,
			user_id: None,
			// Access
			window_access: None,
            process_access: None,
            form_access: None,
            browser_access: None,
            workflow_access: None,
            dashboard_access: None
        }
    }
}

impl Role {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut menu = Role::default();
        menu.id = _id;
        menu
    }
}

impl IndexDocument for Role {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "integer" },
                    "tree_id" : { "type" : "integer" },
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

pub async fn role_from_id(_id: Option<i32>, _language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> Result<Role, String> {
	if _id.is_none() || _id.map(|id| id <= 0).unwrap_or(false) {
		return Err(Error::new(ErrorKind::InvalidData.into(), "Role Identifier is Mandatory").to_string());
	}
    let mut _document = Role::from_id(_id);

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
			let menu: Role = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", menu.id);
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

pub async fn roles(
	_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>,
	_search_value: Option<&String>, _page_number: Option<&String>, _page_size: Option<&String>
) -> Result<RoleListResponse, std::io::Error> {
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

    let mut _document = Role::default();
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
            let mut roles_list: Vec<Role> = vec![];
            for value in values {
				let menu: Role = serde_json::from_value(value).unwrap();
				roles_list.push(menu.to_owned());
            }
            Ok(RoleListResponse {
                roles: Some(roles_list)
            })
        },
		Err(error) => {
			Err(Error::new(ErrorKind::InvalidData.into(), error))
		}
    }
}
