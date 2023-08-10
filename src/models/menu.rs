use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;

use crate::controller::opensearch::IndexDocument;
extern crate diesel;

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct MenuDocument {
    pub menu: Option<Menu>
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
            children: None 
        }
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
        "menu".to_string()
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
    pub help: Option<bool>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Process {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<bool>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Form {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<bool>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browse {
    pub uuid: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<bool>,
}