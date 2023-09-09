use std::env;

use opensearch::{OpenSearch, IndexParts, DeleteParts, SearchParts, GetParts};
use opensearch::http::Url;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::indices::{IndicesGetParts, IndicesCreateParts, IndicesDeleteParts};
use serde_json::Value;

pub trait IndexDocument: Sync {
    //  A index definition for mapping
    fn mapping(self: &Self) -> serde_json::Value;
    //  Get data for insert
    fn data(self: &Self) -> serde_json::Value;
    //  Get index name for create and delete index definition
    fn index_name(self: &Self) -> String;
    //  Get Unique ID
    fn id(self: &Self) -> String;
    //  Make a search based on _search_value
    fn find(self: &Self, _search_value: String) -> serde_json::Value;
}

pub fn create_opensearch_client() -> Result<OpenSearch, String> {
    let opensearch_url =  match env::var("OPENSEARCH_URL") {
        Ok(value) => value.clone(),
        Err(_) => {
            log::info!("Variable `OPENSEARCH_URL` Not found from enviroment, loaded with `default` value");
            "http://localhost:9200".to_owned()
        }.to_owned(),
    };
    let url = match Url::parse(&opensearch_url) {
        Ok(value) => value,
        Err(error) => {
            return Err(error.to_string());
        },
    };
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = match TransportBuilder::new(conn_pool)
        .disable_proxy()
        // .auth(Credentials::Basic("admin".to_owned(), "admin".to_owned()))
        .build() {
            Ok(value) => value,
            Err(error) => {
                return Err(error.to_string());
            },
        };
    Ok(OpenSearch::new(transport))
}

pub async fn exists_index(_index_name: String) -> Result<bool, String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Get data
    let _response = client.indices()
        .get(IndicesGetParts::Index(&[&_index_name]))
        .send().await;
    let response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if response.status_code().is_success() {
        Ok(true)
    } else {
        Err(format!("Index {:?} Not Found", _index_name))
    }
}

pub async fn create_index_definition(_index: &dyn IndexDocument) -> Result<bool, String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Get data
    let _response = client.indices()
        .get(IndicesGetParts::Index(&[&_index.index_name()]))
        .send().await;
    let response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        // Create an index
        let _response = client
        .indices()
        .create(IndicesCreateParts::Index(&_index.index_name()))
        .body(_index.mapping())
        .send()
        .await;
        //  
        match _response {
            Ok(value) => {
                if value.status_code().is_success() {
                    log::info!("Index created: {:?}", _index.index_name());
                } else {
                    return Err(format!("Error creating index {:?}({:?})", _index.index_name(), value.text().await));    
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
                return Err(error.to_string());
            }
        }
    }
    Ok(true)
}

pub async fn delete_index_definition(_index: &dyn IndexDocument) -> Result<bool, String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Get data
    let _response = client.indices()
        .get(IndicesGetParts::Index(&[&_index.index_name()]))
        .send().await;
    let response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if response.status_code().is_success() {
        // Create an index
        let _response = client
        .indices()
        .delete(IndicesDeleteParts::Index(&[&_index.index_name()]))
        .send()
        .await;
        //  
        match _response {
            Ok(value) => {
                if value.status_code().is_success() {
                    log::info!("Index deleted: {:?}", _index.index_name());
                } else {
                    return Err(format!("Error deleting index {:?}({:?})", _index.index_name(), value.status_code()));    
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
                return Err(error.to_string());
            }
        }
    }
    Ok(true)
}

pub async fn create(_document: &dyn IndexDocument) -> Result<bool, std::string::String> {
    let client: OpenSearch = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    let _response = create_index_definition(_document).await;
    let _response = match _response {
        Ok(_) => true,
        Err(error) => {
            log::error!("{:?}", error);
            false
        }
    };
    match get_by_id(_document).await {
        Ok(_) => {
            match delete(_document).await {
                Ok(_) => {},
                Err(error) => log::error!("{:?}", error),
            };
        },
        Err(_) => {},
    };
    //  Create
    let _response = client
        .index(IndexParts::IndexId(&_document.index_name(), &_document.id()))
        .body(_document.data())
        .send().await;
    let _response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !_response.status_code().is_success() {
        return Err(format!("Error inserting record {:?} {:?} {:?}", _document.index_name(), _document.id(), _response.text().await));
    }
    Ok(true)
}

pub async fn delete(_document: &dyn IndexDocument) -> Result<bool, std::string::String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Create
    let _response = client
        .delete(DeleteParts::IndexId(&_document.index_name(), &_document.id()))
        .send().await;
    match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    Ok(true)
}

pub async fn find(_document: &dyn IndexDocument, _search_value: String, _from: i64, _size: i64) -> Result<Vec<Value>, std::string::String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Create
    let _response = client
        .search(SearchParts::Index(&[&_document.index_name()]))
        .from(_from)
        .size(_size)
        .body(_document.find(_search_value))
        .send().await;
    let response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        return Err(format!("Error finding record {:?}", response.text().await));
    }
    let response_body = match response.json::<Value>().await {
        Ok(response) => response,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        },
    };
    let mut list: Vec::<Value> = Vec::new();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        let value = hit["_source"].to_owned();
        list.push(value)
    }
    Ok(list)
}

pub async fn get_by_id(_document: &dyn IndexDocument) -> Result<Value, std::string::String> {
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Create
    let _response = client
        .get(GetParts::IndexId(&_document.index_name(), &_document.id()))
        .send().await;
    let _response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !_response.status_code().is_success() {
        return Err(format!("Error finding record by ID {:?}", _response.text().await));
    }
    let response_body = match _response.json::<Value>().await {
        Ok(response) => {
            response["_source"].to_owned()
        },
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        },
    };
    Ok(response_body)
}