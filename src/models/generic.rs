use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use serde_json::json;

use crate::controller::opensearch::IndexDocument;

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct GenericDocument {
    pub document: Option<Generic>
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Generic {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub display_value: Option<String>,
    pub index_value: Option<String>
}

impl Default for Generic {
    fn default() -> Self {
        Self { 
            uuid: None, 
            internal_id: None,
            id: None, 
            display_value: None,
            index_value: None
        }
    }
}

impl IndexDocument for Generic {
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
                    "description" : { "type" : "text" }
                }
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
