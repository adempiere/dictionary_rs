use std::env;

use opensearch::http::response::Response;
use opensearch::{OpenSearch, IndexParts, DeleteParts, SearchParts, GetParts};
use opensearch::http::Url;
use opensearch::http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder};
use opensearch::indices::{IndicesGetParts, IndicesCreateParts, IndicesDeleteParts};
use salvo::http::StatusCode;
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
	let opensearch_url: String =  match env::var("OPENSEARCH_URL") {
        Ok(value) => value.clone(),
        Err(_) => {
            log::warn!("Variable `OPENSEARCH_URL` Not found from enviroment, loaded with `default` value");
            "http://localhost:9200".to_owned()
        }.to_owned(),
    };
	let url: Url = match Url::parse(&opensearch_url) {
        Ok(value) => value,
        Err(error) => {
            return Err(error.to_string());
        },
    };
	let conn_pool: SingleNodeConnectionPool = SingleNodeConnectionPool::new(url);
	let transport: Transport = match TransportBuilder::new(conn_pool)
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
	let client: OpenSearch = create_opensearch_client()?;

    //  Get data
	let _response: Result<opensearch::http::response::Response, opensearch::Error> = client.indices()
        .get(IndicesGetParts::Index(&[&_index_name]))
		.send()
		.await
	;
	let response: Response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };

	let status: StatusCode = response.status_code();
	if status.is_success() {
        Ok(true)
	} else if status.as_u16() == 404 {
		// Not exists
		Ok(false)
    } else {
        Err(format!("Index {:?} Not Found", _index_name))
    }
}

pub async fn create_index_definition(_index: &dyn IndexDocument) -> Result<bool, String> {
	let index_name: String = _index.index_name();
	if exists_index(index_name.clone()).await? {
		log::debug!("Index {:?} already exist, skipping creation.", index_name);
		return Ok(true);
	}

	let client: OpenSearch = create_opensearch_client()?;

    //  Get data
	let _response: Result<opensearch::http::response::Response, opensearch::Error> = client.indices()
        .get(IndicesGetParts::Index(&[&_index.index_name()]))
		.send()
		.await
	;
	let response: Response = match _response {
		Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        // Create an index
		let _response: Result<opensearch::http::response::Response, opensearch::Error> = client
			.indices()
			.create(IndicesCreateParts::Index(&_index.index_name()))
			.body(_index.mapping())
			.send()
			.await
		;
		//
        match _response {
            Ok(value) => {
                if value.status_code().is_success() {
                    log::info!("Index created: {:?}", _index.index_name());
                } else {
					return Err(
						format!("Error creating index {:?} ({:?})", _index.index_name(), value.text().await)
					);
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
	let index_name: String = _index.index_name();
	if !exists_index(index_name.clone()).await? {
		log::warn!("Index {:?} does not exist, skipping deletion.", index_name);
		return Ok(true);
	}

	let client: OpenSearch = create_opensearch_client()?;

    //  Get data
	let _response: Result<opensearch::http::response::Response, opensearch::Error> = client.indices()
        .get(IndicesGetParts::Index(&[&_index.index_name()]))
		.send()
		.await
	;
	let response: Response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if response.status_code().is_success() {
        // Create an index
		let _response: Result<Response, opensearch::Error> = client
			.indices()
			.delete(IndicesDeleteParts::Index(&[&_index.index_name()]))
			.send()
			.await
		;
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
	let client: OpenSearch = create_opensearch_client()?;

	let _response: Result<bool, String> = create_index_definition(_document).await;
	let _response: bool = match _response {
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
	// Create
	let _response: Result<Response, opensearch::Error> = client
        .index(IndexParts::IndexId(&_document.index_name(), &_document.id()))
        .body(_document.data())
		.send()
		.await
	;
	let _response: Response = match _response {
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
	let client: OpenSearch = create_opensearch_client()?;

	// Delete
	let _response: Result<Response, opensearch::Error> = client
        .delete(DeleteParts::IndexId(&_document.index_name(), &_document.id()))
		.send()
		.await
	;

    match _response {
        Ok(value) => {
			let status: StatusCode = value.status_code();
			// For the ‘delete’ operation, the OpenSearch library often
			// considers 404 as ‘success’ if the document does not exist.
			if !status.is_success() && status.as_u16() != 404 {
				return Err(
					format!("Error deleting record {:?} {:?} {:?}", _document.index_name(), _document.id(), value.text().await)
				);
			}
			value
		},
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    Ok(true)
}

pub async fn find(_document: &dyn IndexDocument, _search_value: String, _from: i64, _size: i64) -> Result<Vec<Value>, std::string::String> {
	let client: OpenSearch = create_opensearch_client()?;

	// Get
	let _response: Result<opensearch::http::response::Response, opensearch::Error> = client
        .search(SearchParts::Index(&[&_document.index_name()]))
        .from(_from)
        .size(_size)
        .body(_document.find(_search_value))
        .send()
		.await
	;
	let response: Response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        return Err(format!("Error finding record {:?}", response.text().await));
    }
	let response_body: Value = match response.json::<Value>().await {
        Ok(response) => response,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        },
    };

	let mut list: Vec::<Value> = Vec::new();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
		let value: Value = hit["_source"].to_owned();
        list.push(value)
    }
    Ok(list)
}

pub async fn find_from_dsl_body(_index_name: String, _body: serde_json::Value, _from: i64, _size: i64) -> Result<Vec<Value>, std::string::String> {
	let client: OpenSearch = create_opensearch_client()?;

    //  Get
	let _response: Result<opensearch::http::response::Response, opensearch::Error> = client
        .search(SearchParts::Index(&[&_index_name]))
        .from(_from)
        .size(_size)
        .body(_body)
        .send()
		.await
	;
	let response: Response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !response.status_code().is_success() {
        return Err(format!("Error finding record {:?}", response.text().await));
    }
	let response_body: Value = match response.json::<Value>().await {
        Ok(response) => response,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        },
    };

	let hits: &Vec<Value> = response_body["hits"]["hits"].as_array().unwrap();
	let mut list: Vec::<Value> = Vec::new();
	for hit in hits {
		let source: &Value = &hit["_source"];
		let value: Value = source.to_owned();
		list.push(value)
	}

    Ok(list)
}

pub async fn get_by_id(_document: &dyn IndexDocument) -> Result<Value, std::string::String> {
	let client: OpenSearch = create_opensearch_client()?;

	// Get
	let _response: Result<Response, opensearch::Error> = client
        .get(GetParts::IndexId(&_document.index_name(), &_document.id()))
		.send()
		.await
	;
	let _response: Response = match _response {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:?}", error);
            return Err(error.to_string());
        }
    };
    if !_response.status_code().is_success() {
        return Err(format!("Error finding record by ID {:?}", _response.text().await));
    }
	let response_body: Value = match _response.json::<Value>().await {
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
