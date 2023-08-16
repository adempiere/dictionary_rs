use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::controller::opensearch::{IndexDocument, get_by_id, find};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct MenuDocument {
    pub menu: Option<Menu>
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
    pub action: Option<String>,
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
    pub window: Option<Window>,
    pub process: Option<Process>,
    pub form: Option<Form>,
    pub browse: Option<Browse>,
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
            action: None, 
            window: None, 
            process: None, 
            form: None, 
            browse: None,
            children: None,
            client_id: None,
            index_value: None,
            language: None,
            role_id: None,
            user_id: None
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
        json!({
            "query": {
                "multi_match": {
                    "query": _search_value,
                    "fields": ["name^2", "description"]
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

pub async fn menu_from_id(_id: Option<i32>) -> Result<MenuResponse, String> {
    let mut _document = Menu::from_id(_id);
    let _menu_document: &dyn IndexDocument = &_document;
    match get_by_id(_menu_document).await {
        Ok(value) => {
            let menu: Menu = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", menu);
            Ok(MenuResponse {
                menu: Some(menu)
            })
        },
        Err(error) => {
            log::warn!("{}", error);
            Err(error)
        },
    }
}

fn default_menu(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>) -> String {
    let mut _default_menu: String = "menu".to_owned();
    _default_menu.push_str("_");
    _default_menu.push_str(_language.unwrap());
    _default_menu.push_str("_");
    _default_menu.push_str(_client_id.unwrap());
    _default_menu.push_str("_");
    _default_menu.push_str(_role_id.unwrap());
    _default_menu.to_lowercase()
}

fn user_menu(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>) -> String {
    let mut _default_menu = default_menu(_language, _client_id, _role_id);
    _default_menu.push_str("_");
    _default_menu.push_str(_user_id.unwrap());
    _default_menu.to_lowercase()
}

pub async fn menus(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _search_value: Option<&String>) -> Result<MenuListResponse, std::io::Error> {
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
    let _default_menu = match _user_id {
        Some(_) => user_menu(_language, _client_id, _role_id, _user_id),
        None => default_menu(_language, _client_id, _role_id)
    };
    let _search_value = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };
    log::info!("Index to search {:}", _default_menu);
    let mut _document = Menu::default();
    _document.index_value = Some(_default_menu);
    let _menu_document: &dyn IndexDocument = &_document;
    match find(_menu_document, _search_value, 0, 10).await {
        Ok(values) => {
            let mut menus: Vec<Menu> = vec![];
            for value in values {
                let menu: Menu = serde_json::from_value(value).unwrap();
                menus.push(menu.to_owned());
            }
            Ok(MenuListResponse {
                menus: Some(menus)
            })
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
    // Ok(MenuResponse::default())
}