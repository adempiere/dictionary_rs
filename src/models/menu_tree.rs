use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::controller::opensearch::{IndexDocument, get_by_id};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct MenuTreeDocument {
    pub document: Option<MenuTree>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuTreeResponse {
    pub menu: Option<MenuTree>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuTreeListResponse {
    pub menus: Option<Vec<MenuTree>>
}

impl Default for MenuTreeResponse {
    fn default() -> Self {
        MenuTreeResponse { 
            menu: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct MenuTree {
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub node_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub sequence: Option<i32>,
    // index
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    // Tree menu childs
    pub children: Option<Vec<MenuTree>>
}

impl Default for MenuTree {
    fn default() -> Self {
        Self { 
            id: None, 
            internal_id: None,
            node_id: None,
            parent_id: None, 
            sequence: None, 
            // index
			index_value: None,
			language: None,
			client_id: None,
			role_id: None,
			user_id: None,
			// Tree menu childs
			children: None
        }
    }
}

impl MenuTree {
    pub fn from_id(_id: Option<String>) -> Self {
        let mut menu = MenuTree::default();
        menu.id = _id;
        menu
    }
}

impl IndexDocument for MenuTree {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "text" },
                    "parent_id" : { "type" : "integer" },
                    "sequence" : { "type" : "integer" }                }
            }
        })
    }

    fn data(self: &Self) -> serde_json::Value {
        json!(self)
    }

    fn id(self: &Self) -> String {
        self.id.to_owned().unwrap()
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

pub async fn menu_tree_from_id(_id: Option<String>, _dictionary_code: Option<&String>) -> Result<MenuTree, std::io::Error> {
	if _id.is_none() || _id.as_deref().map_or(false, |s| s.trim().is_empty()) {
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "MenuTree Identifier is Mandatory")
		);
	}
    let mut _document = MenuTree::from_id(_id);

	let mut _index_name = "menu_tree".to_string();
	if let Some(code) = _dictionary_code {
		if !code.trim().is_empty() {
			_index_name.push_str("_");
			_index_name.push_str(code);
		}
	}
	log::info!("Index to search {:}", _index_name);

	_document.index_value = Some(_index_name);
    let _menu_document: &dyn IndexDocument = &_document;
    match get_by_id(_menu_document).await {
        Ok(value) => {
			let mut menu: MenuTree = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", menu.id);
			// sort menu children nodes by sequence
			if let Some(ref mut children) = menu.children {
				children.sort_by_key(|child| child.sequence.clone().unwrap_or(0));
			}
            Ok(menu)
        },
        Err(error) => {
			log::error!("{}", error);
            Err(Error::new(ErrorKind::InvalidData.into(), error))
        },
    }
}
