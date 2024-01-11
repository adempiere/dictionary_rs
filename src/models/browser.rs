use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;
use std::{io::ErrorKind, io::Error};

use crate::{controller::opensearch::{IndexDocument, get_by_id, find, exists_index}, models::{user_index, role_index, default_index, language_index, client_index}};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct BrowserDocument {
    pub document: Option<Browser>
}

#[derive(Serialize, Debug, Clone)]
pub struct BrowserResponse {
    pub browser: Option<Browser>
}

#[derive(Serialize, Debug, Clone)]
pub struct BrowserListResponse {
    pub browsers: Option<Vec<Browser>>
}

impl Default for BrowserResponse {
    fn default() -> Self {
        BrowserResponse { 
            browser: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browser {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub value: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub entity_type: Option<String>,
    pub access_level: Option<String>,
    pub index_value: Option<String>,
    pub language: Option<String>,
    pub client_id: Option<i32>,
    pub role_id: Option<i32>,
    pub user_id: Option<i32>,
    pub is_collapsible_by_default: Option<bool>,
    pub is_deleteable: Option<bool>,
    pub is_execute_query_by_default: Option<bool>,
    pub is_selected_by_default: Option<bool>,
    pub is_show_total: Option<bool>,
    pub is_updateable: Option<bool>,
    pub process: Option<Process>,
    pub window: Option<Window>,
    pub table: Option<Table>,
    pub display_fields: Option<Vec<BrowserField>>,
    pub criteria_fields: Option<Vec<BrowserField>>,
    pub identifier_fields: Option<Vec<BrowserField>>,
    pub order_fields: Option<Vec<BrowserField>>,
    pub editable_fields: Option<Vec<BrowserField>>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct BrowserField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub value: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub entity_type: Option<String>,
    pub column_name: Option<String>,
    pub element_name: Option<String>,
    pub default_value: Option<String>,
    pub default_value_to: Option<String>,
    pub is_range: Option<bool>,
    pub is_mandatory: Option<bool>,
    pub is_info_only: Option<bool>,
    pub display_logic: Option<String>,
    pub value_format: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub sequence: Option<i32>,
    pub grid_sequence: Option<i32>,
    pub is_displayed: Option<bool>,
    pub is_query_criteria: Option<bool>,
    pub is_order_by: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_key: Option<bool>,
    pub is_identifier: Option<bool>,
    pub display_type: Option<i32>,
    pub reference_value_id: Option<i32>,
    pub validation_id: Option<i32>,
    pub context_column_names: Option<Vec<String>>,
    pub dependent_fields: Option<Vec<DependendField>>
}

impl Default for Browser {
    fn default() -> Self {
        Self { 
            uuid: None, 
            id: None, 
            value: None, 
            name: None, 
            description: None, 
            help: None, 
            access_level: None,
            process: None,
            criteria_fields: None,
            display_fields: None,
            editable_fields: None,
            entity_type: None,
            identifier_fields: None,
            is_collapsible_by_default: None,
            is_deleteable: None,
            is_execute_query_by_default: None,
            is_selected_by_default: None,
            is_show_total: None,
            is_updateable: None,
            order_fields: None,
            table: None,
            window: None,
            client_id: None,
            index_value: None,
            language: None,
            role_id: None,
            user_id: None
        }
    }
}

impl Browser {
    pub fn from_id(_id: Option<i32>) -> Self {
        let mut browser = Browser::default();
        browser.id = _id;
        browser
    }
}

impl IndexDocument for Browser {
    fn mapping(self: &Self) -> serde_json::Value {
        json!({
            "mappings" : {
                "properties" : {
                    "uuid" : { "type" : "text" },
                    "id" : { "type" : "integer" },
                    "value" : { "type" : "text" },
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
            None => "browser".to_string(),
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
pub struct Process {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
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
pub struct Table {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub table_name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub is_document: Option<bool>,
    pub is_deleteable: Option<bool>,
    pub is_view: Option<bool>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct DependendField {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub column_name: Option<String>,
    pub parent_id: Option<i32>,
    pub parent_uuid: Option<String>,
}

pub async fn browser_from_id(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _id: Option<i32>) -> Result<BrowserResponse, String> {
    let mut _document = Browser::from_id(_id);
    let _index_name = get_index_name(_language, _client_id, _role_id, _user_id).await.expect("Error getting index");
    log::info!("Index to search {:}", _index_name);
    _document.index_value = Some(_index_name);
    let _menu_document: &dyn IndexDocument = &_document;
    match get_by_id(_menu_document).await {
        Ok(value) => {
            let browser: Browser = serde_json::from_value(value).unwrap();
            log::info!("Finded Value: {:?}", browser.id);
            Ok(BrowserResponse {
                browser: Some(browser)
            })
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
    let _index = "browser".to_string();
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

pub async fn browsers(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _user_id: Option<&String>, _search_value: Option<&String>) -> Result<BrowserListResponse, std::io::Error> {
    let _search_value = match _search_value {
        Some(value) => value.clone(),
        None => "".to_owned()
    };
    let _index_name = get_index_name(_language, _client_id, _role_id, _user_id).await.expect("Error getting index");
    log::info!("Index to search {:}", _index_name);
    let mut _document = Browser::default();
    _document.index_value = Some(_index_name);
    let _menu_document: &dyn IndexDocument = &_document;
    match find(_menu_document, _search_value, 0, 10).await {
        Ok(values) => {
            let mut menus: Vec<Browser> = vec![];
            for value in values {
                let menu: Browser = serde_json::from_value(value).unwrap();
                menus.push(menu.to_owned());
            }
            Ok(BrowserListResponse {
                browsers: Some(menus)
            })
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
    }
    // Ok(BrowserResponse::default())
}