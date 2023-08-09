use std::env;

use opensearch::{OpenSearch, IndexParts};
use opensearch::http::Url;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::indices::{IndicesGetParts, IndicesCreateParts};

pub trait IndexDocument: Sync {
    fn mapping(self: &Self) -> serde_json::Value;
    fn data(self: &Self) -> serde_json::Value;
    fn index_name(self: &Self) -> String;
    fn id(self: &Self) -> String;
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
                    return Err(format!("Error creating index {:?}({:?})", _index.index_name(), value.status_code()));    
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
    let client = match create_opensearch_client() {
        Ok(client_value) => client_value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    //  Create
    let _response = client
        .index(IndexParts::IndexId(&_document.index_name(), &_document.id()))
        .body(_document.data())
        .send().await;
    let response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        return Err("Error inserting record".to_owned());
    }
    Ok(true)
}