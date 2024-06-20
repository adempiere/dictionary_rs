use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::controller::opensearch::{IndexDocument, get_by_id, exists_index};

use super::client_index_only;

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
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tree_id: Option<i32>,
    pub tree_uuid: Option<String>,
    // index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
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
            internal_id: None,
            uuid: None, 
            name: None, 
            description: None, 
            tree_id: None, 
            tree_uuid: None, 
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
    pub fn from_id(_id: Option<&String>) -> Self {
        let mut menu = Role::default();
        menu.uuid = _id.cloned();
        menu
    }
}

impl IndexDocument for Role {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "text" },
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
        self.uuid.to_owned().unwrap()
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

pub async fn role_from_id(_id: Option<&String>, _client_id: Option<&String>) -> Result<Role, String> {
	if _id.is_none() {
		return Err(Error::new(ErrorKind::InvalidData.into(), "Role Identifier is Mandatory").to_string());
	}
    let mut _document = Role::from_id(_id);

	let _index_name = match get_index_name(_client_id).await {
		Ok(index_name) => index_name,
		Err(error) => {
			log::error!("Index name error: {:?}", error.to_string());
			return Err(error.to_string())
		}
	};
	log::info!("Index to search {:}", _index_name);

	_document.index_value = Some(_index_name);
    let _role_document: &dyn IndexDocument = &_document;
    match get_by_id(_role_document).await {
        Ok(value) => {
			let role: Role = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", role.id);
            Ok(role)
        },
        Err(error) => {
			log::error!("{}", error);
            Err(error)
        },
    }
}

async fn get_index_name(_client_id: Option<&String>) -> Result<String, std::io::Error> {
	if _client_id.is_none() {
		return Err(Error::new(ErrorKind::InvalidData.into(), "Client is Mandatory"));
	}

    let _base_index: String = "role".to_string();
	let _index = client_index_only(_base_index.to_owned(), _client_id);

	//  Find index
	match exists_index(_index.to_owned()).await {
		Ok(_) => {
			log::info!("Find with role index `{:}`", _index);
			Ok(_index)
		},
		Err(_) => {
			log::error!("No index found `{:}`", _index);
            return Err(Error::new(ErrorKind::InvalidData.into(), "No Index Found"))
		}
	}
}
