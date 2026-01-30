use std::env;
use dictionary_rs::{controller::{kafka::create_consumer, opensearch::{create, delete, IndexDocument}}, models::{browser::{browser_from_id, browsers, BrowserDocument}, form::{form_from_id, forms, FormDocument}, menu::allowed_menu, menu_item::MenuItemDocument, menu_tree::MenuTreeDocument, process::{process_from_id, processes, ProcessDocument}, role::RoleDocument, window::{window_from_id, windows, WindowDocument}}};
use dotenv::dotenv;
use rdkafka::{Message, consumer::{CommitMode, Consumer}};
use salvo::{conn::tcp::TcpAcceptor, cors::Cors, http::header, hyper::Method, prelude::*};
extern crate serde_json;
use serde::Serialize;
use simple_logger::SimpleLogger;
use futures::future::join_all;

#[tokio::main]
async fn main() {
	dotenv().ok();
	SimpleLogger::new().env().init().unwrap();

	let port: String = match env::var("PORT") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `PORT` Not found from enviroment, as default 7878");
			"7878".to_owned()
		}.to_owned()
	};

	let host: String = "0.0.0.0:".to_owned() + &port;
	log::info!("Server Address: {:?}", host.clone());
	let acceptor: TcpAcceptor = TcpListener::new(host).bind().await;

	let mut futures: Vec<tokio::task::JoinHandle<()>> = Vec::new();
	futures.push(
		tokio::spawn(
			async move { Server::new(acceptor).serve(routes()).await; }
		)
	);

	// Kafka Queue
	let kafka_enabled: String = match env::var("KAFKA_ENABLED") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `KAFKA_ENABLED` Not found from enviroment, as default Y");
			"Y".to_owned()
		}.to_owned()
	};
	if kafka_enabled.trim().eq("Y") {
		log::info!("Kafka Consumer is enabled");
		futures.push(
			tokio::spawn(
				async move { consume_queue().await; }
			)
		);
	} else {
		log::info!("Kafka Consumer is disabled");
	}

	join_all(futures).await;
}

fn routes() -> Router {
	// TODO: Add support to allow requests from multiple origin
	let allowed_origin: String = match env::var("ALLOWED_ORIGIN") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `ALLOWED_ORIGIN` Not found from enviroment");
			"*".to_owned()
		}.to_owned()
	};

	let allow_methods: Vec<Method> = vec![
		Method::OPTIONS,
		Method::GET
	];
	let allow_headers: Vec<header::HeaderName> = vec![
		header::ACCESS_CONTROL_REQUEST_METHOD,
		header::ACCESS_CONTROL_REQUEST_HEADERS,
		header::AUTHORIZATION
	];
	// Send Device Info
	let cors_handler = Cors::new()
		.allow_origin(&allowed_origin.to_owned())
		.allow_methods(allow_methods)
		.allow_headers(allow_headers)
		.into_handler()
	;

	let router: Router = Router::new()
		.hoop(cors_handler)
		// /	root path
		.options(options_response)
		.get(get_system_info)
		.push(
			// /api
			Router::with_path("api")
				.options(options_response)
				.get(get_system_info)
				.push(
					// /api/security/menus
					Router::with_path("security/menus")
						.options(options_response)
						.get(get_allowed_menu)
				)
				.push(
					// /api/dictionary
					Router::with_path("dictionary")
						.push(
							// /api/dictionary/system-info
							Router::with_path("system-info")
								.options(options_response)
								.get(get_system_info)
						)
						.push(
							// /api/dictionary/browsers/
							Router::with_path("browsers")
								.options(options_response)
								.get(get_browsers)
								.push(
									// /api/dictionary/browsers/:id
									Router::with_path("{id}")
										.options(options_response)
										.get(get_browsers)
								)
						)
						.push(
							// /api/dictionary/forms/
							Router::with_path("forms")
								.options(options_response)
								.get(get_forms)
								.push(
									// /api/dictionary/forms/:id
									Router::with_path("{id}")
										.options(options_response)
										.get(get_forms)
								)
						)
						.push(
							// /api/dictionary/processes
							Router::with_path("processes")
								.options(options_response)
								.get(get_processes)
								.push(
									// /api/dictionary/processes/:id
									Router::with_path("{id}")
										.options(options_response)
										.get(get_processes)
								)
						)
						.push(
							// /api/dictionary/windows/
							Router::with_path("windows")
								.options(options_response)
								.get(get_windows)
								.push(
									// /api/dictionary/windows/:id
									Router::with_path("{id}")
										.options(options_response)
										.get(get_windows)
								)
						)
				)
		)
	;

	log::info!("{:#?}", router);
	router
}

#[handler]
async fn options_response<'a>(_req: &mut Request, _res: &mut Response) {
	_res.status_code(StatusCode::NO_CONTENT);
}

#[derive(Serialize)]
struct SystemInfoResponse {
	version: String,
	is_kafka_enabled: bool,
	kafka_queues: String,
}

#[handler]
async fn get_system_info<'a>(_req: &mut Request, _res: &mut Response) {
	let version: String = match env::var("VERSION") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `VERSION` Not found from enviroment, as default `1.0.0-dev`");
			"1.0.0-dev".to_owned()
		}.to_owned()
	};

	// Kafka Queue
	let kafka_enabled: String = match env::var("KAFKA_ENABLED") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `KAFKA_ENABLED` Not found from enviroment, as default Y");
			"Y".to_owned()
		}.to_owned()
	};
	let kafka_queues: String = match env::var("KAFKA_QUEUES") {
		Ok(value) => value.clone(),
		Err(_) => {
			log::warn!("Variable `KAFKA_QUEUES` Not found from enviroment, loaded with `default` value");
			"browser form process window menu_item menu_tree role".to_owned()
		}.to_owned()
	};

	let system_info_response: SystemInfoResponse = SystemInfoResponse {
		version: version.to_string(),
		is_kafka_enabled: kafka_enabled.trim().eq("Y"),
		kafka_queues: kafka_queues
	};

	_res.status_code(StatusCode::OK)
		.render(
			Json(system_info_response)
		)
	;
}


#[derive(Serialize)]
struct ErrorResponse {
	status: u16,
	message: String
}

#[handler]
async fn get_forms<'a>(_req: &mut Request, _res: &mut Response) {
	let mut _id: Option<String> = _req.param::<String>("id");
	if _id.is_none() {
		// fill with query url
		_id = _req.queries().get("id").map(|s| s.to_owned());
	}
	log::debug!("Get by ID: {:?}", _id);

	let _language: Option<&String> = _req.queries().get("language");
	let _dictionary_code: Option<&String> = _req.queries().get("dictionary_code");
	let _search_value: Option<&String> = _req.queries().get("search_value");
	if _id.is_some() {
		match form_from_id(_id, _language, _dictionary_code).await {
			Ok(form) => _res.render(Json(form)),
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
		}
	} else {
		let _search_value: Option<&String> = _req.queries().get("search_value");
		match forms(_language, _search_value, _dictionary_code).await {
			Ok(forms_list) => {
				_res.render(Json(forms_list));
			},
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
		}
	}
}

#[handler]
async fn get_allowed_menu<'a>(_req: &mut Request, _res: &mut Response) {
	let _language: Option<&String> = _req.queries().get("language");
	let _client_id: Option<&String> = _req.queries().get("client_id");
	let _role_id: Option<&String> = _req.queries().get("role_id");
	let _dictionary_code: Option<&String> = _req.queries().get("dictionary_code");
	match allowed_menu(_language, _client_id, _role_id, _dictionary_code).await {
        Ok(menu) => _res.render(Json(menu)),
        Err(error) => {
			let error_response: ErrorResponse = ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.into(),
                message: error.to_string()
            };
            _res.render(
                Json(error_response)
            );
            _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[handler]
async fn get_processes<'a>(_req: &mut Request, _res: &mut Response) {
	let mut _id: Option<String> = _req.param::<String>("id");
	if _id.is_none() {
		// fill with query url
		_id = _req.queries().get("id").map(|s| s.to_owned());
	}
	log::debug!("Get by ID: {:?}", _id);

	let _language: Option<&String> = _req.queries().get("language");
	let _dictionary_code: Option<&String> = _req.queries().get("dictionary_code");
	let _search_value: Option<&String> = _req.queries().get("search_value");
	if _id.is_some() {
		match process_from_id(_id, _language, _dictionary_code).await {
            Ok(process) => _res.render(Json(process)),
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
        }
    } else {
        match processes(_language, _search_value, _dictionary_code).await {
            Ok(processes_list) => {
                _res.render(Json(processes_list));
            },
			Err(error) => {
				let error_response = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
        }
    }
}

#[handler]
async fn get_browsers<'a>(_req: &mut Request, _res: &mut Response) {
	let mut _id: Option<String> = _req.param::<String>("id");
	if _id.is_none() {
		// fill with query url
		_id = _req.queries().get("id").map(|s| s.to_owned());
	}
	log::debug!("Get by ID: {:?}", _id);

	let _language: Option<&String> = _req.queries().get("language");
	let _dictionary_code: Option<&String> = _req.queries().get("dictionary_code");
	let _search_value: Option<&String> = _req.queries().get("search_value");
	if _id.is_some() {
		match browser_from_id(_id, _language, _dictionary_code).await {
            Ok(browser) => _res.render(Json(browser)),
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
        }
    } else {
        match browsers(_language, _search_value, _dictionary_code).await {
            Ok(browsers_list) => {
                _res.render(Json(browsers_list));
            },
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
        }
    }
}

#[handler]
async fn get_windows<'a>(_req: &mut Request, _res: &mut Response) {
	let mut _id: Option<String> = _req.param::<String>("id");
	if _id.is_none() {
		_id = _req.queries().get("id").map(|s| s.to_owned());
	}
	log::debug!("Get by ID: {:?}", _id);

	let _language: Option<&String> = _req.queries().get("language");
	let _dictionary_code: Option<&String> = _req.queries().get("dictionary_code");
	let _search_value: Option<&String> = _req.queries().get("search_value");
	if _id.is_some() {
		match window_from_id(_id, _language, _dictionary_code).await {
            Ok(window) => _res.render(Json(window)),
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
			}
        }
    } else {
        match windows(_language, _search_value, _dictionary_code).await {
            Ok(windows_list) => {
                _res.render(Json(windows_list));
            },
			Err(error) => {
				let error_response: ErrorResponse = ErrorResponse {
					status: StatusCode::INTERNAL_SERVER_ERROR.into(),
					message: error.to_string()
				};
				_res.render(
					Json(error_response)
				);
				_res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}

async fn consume_queue() {
	let kafka_host: String = match env::var("KAFKA_HOST") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `KAFKA_HOST` Not found from enviroment, loaded from local IP");
			"127.0.0.1:9092".to_owned()
		}.to_owned(),
	};
	log::info!("Kafka queue to Subscribe: {:?}", kafka_host.to_owned());

	let kafka_group: String = match env::var("KAFKA_GROUP") {
		Ok(value) => value,
		Err(_) => {
			log::warn!("Variable `KAFKA_GROUP` Not found from enviroment, loaded with `default` value");
			"default".to_owned()
		}.to_owned(),
	};
	let kafka_queues: String = match env::var("KAFKA_QUEUES") {
		Ok(value) => value.clone(),
		Err(_) => {
			log::warn!("Variable `KAFKA_QUEUES` Not found from enviroment, loaded with `default` value");
			"browser form process window menu_item menu_tree role".to_owned()
		}.to_owned()
	};

	let topics_list: Vec<&str> = kafka_queues.split_whitespace().collect();
	log::info!("Kafka Topics to Subscribe: {:?}", topics_list.to_owned());

	let consumer_result= create_consumer(&kafka_host, &kafka_group, &topics_list);
    match consumer_result {
        Ok(consumer) => {
            loop {
                match consumer.recv().await {
                    Err(e) => log::error!("Kafka error: {}", e),
                    Ok(message) => {
						let key: &str = match message.key_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
								log::error!("Error while deserializing message key: {:?}", e);
                                ""
                            }
                        };
						let event_type: String = key.replace("\"", "");
						let topic: &str = message.topic();
						if (topics_list.contains(&topic)) == false {
							log::warn!("Topic {:?} not allowed to be processed", topic);
							continue;
						}

						let payload: &str = match message.payload_view::<str>() {
							None => "",
							Some(Ok(s)) => s,
							Some(Err(e)) => {
								log::error!("Error while deserializing message payload: {:?}", e);
								""
							}
						};
                        if topic == "menu_item" {
							let _document: MenuItemDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    MenuItemDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _menu_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _menu_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _menu_document.index_name(), error)
                                }
                            }
                        } else if topic == "menu_tree" {
							let _document: MenuTreeDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    MenuTreeDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _menu_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _menu_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _menu_document.index_name(), error)
                                }
                            }
                        } else if topic == "role" {
							let _document: RoleDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    RoleDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _menu_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _menu_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _menu_document.index_name(), error)
                                }
                            }
                        } else if topic == "process" {
							let _document: ProcessDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    ProcessDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _process_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _process_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _process_document.index_name(), error)
                                }
                            }
                        } else if topic == "browser" {
							let _document: BrowserDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    BrowserDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _browser_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _browser_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _browser_document.index_name(), error)
                                }
                            }
                        } else if topic == "window" {
							let _document: WindowDocument = match serde_json::from_str(payload) {
                                Ok(value) => value,
                                Err(error) => {
                                    log::warn!("Topic: {:?}, {}", topic, error);
                                    WindowDocument {
                                        document: None
                                    }
                                },
                            };
                            if _document.document.is_some() {
                                let _window_document: &dyn IndexDocument = &(_document.document.unwrap());
                                match process_index(event_type, _window_document).await {
                                    Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                    Err(error) => log::warn!("Document: {:?} {}", _window_document.index_name(), error)
                                }
                            }
						} else if topic == "form" {
							let _document: FormDocument = match serde_json::from_str(payload) {
								Ok(value) => value,
								Err(error) => {
									log::warn!("Topic: {:?}, {}", topic, error);
									FormDocument {
										document: None
									}
								},
							};
							if _document.document.is_some() {
								let _form_document: &dyn IndexDocument = &(_document.document.unwrap());
								match process_index(event_type, _form_document).await {
									Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
									Err(error) => log::warn!("Document: {:?} {}", _form_document.index_name(), error)
								}
							}
                        }
                        // TODO: Add token header
                        // if let Some(headers) = message.headers() {
                        //     for header in headers.iter() {
                        //         log::info!("  Header {:#?}: {:?}", header.key, header.value);
                        //     }
                        // }
                    }
                };
            }
        },
        Err(error) => log::error!("Consume Queue Error {}", error),
    };
}

async fn process_index(_event_type: String, _document: &dyn IndexDocument) -> Result<bool, std::string::String> {
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
