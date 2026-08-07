use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use opensearch::{OpenSearch, http::response::Response, indices::{IndicesCreateParts, IndicesDeleteParts, IndicesGetParts}};
use salvo::http::StatusCode;

use crate::controller::{opensearch_client::create_opensearch_client, opensearch_document::{IndexDocument, create, delete}};

static INDEX_EXISTS_CACHE: OnceLock<RwLock<HashMap<String, bool>>> = OnceLock::new();

/// Cached wrapper around `exists_index` for hot-path lookups (e.g. resolving the
/// role/menu_item index name on every `security/menus` request). Index names are
/// stable at runtime, so the HEAD round-trip to OpenSearch only needs to happen
/// once per index name per process. NOT used by index management operations
/// (`create_index_definition`/`delete_index_definition`), which must see fresh state.
pub async fn exists_index_cached(_index_name: String) -> Result<bool, String> {
	let cache: &RwLock<HashMap<String, bool>> = INDEX_EXISTS_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
	if let Some(&exists) = cache.read().unwrap().get(&_index_name) {
		return Ok(exists);
	}

	let exists: bool = exists_index(_index_name.clone()).await?;
	cache.write().unwrap().insert(_index_name, exists);
	Ok(exists)
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


pub async fn process_index(_event_type: String, _document: &dyn IndexDocument) -> Result<bool, std::string::String> {
	let index_name: String = _document.index_name();
	let id: String = _document.id();
	log::debug!("Event `{:}` into index {:} with id {:} ", _event_type, index_name, id);

	if _event_type.eq("new") {
		match create(_document).await {
			Ok(_) => return Ok(true),
			Err(error) => return Err(error.to_string())
		};  
	} else if _event_type.eq("update") {
		match delete(_document).await {
			Ok(_) => {
				match create(_document).await {
					Ok(_) => return Ok(true),
					Err(error) => return Err(error.to_string())
				}
			},
			Err(error) => return Err(error.to_string())
		};
	} else if _event_type.eq("delete") {
		match delete(_document).await {
			Ok(_) => return Ok(true),
			Err(error) => return Err(error.to_string())
		};
	}
	Ok(true)
}
