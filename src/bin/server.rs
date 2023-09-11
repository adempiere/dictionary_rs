use std::env;
use opensearch_gateway_rs::{models::{menu::{menu_from_id, menus, MenuDocument}, process::{ProcessDocument, process_from_id, processes}, browser::{BrowserDocument, browsers, browser_from_id}, window::{WindowDocument, windows, window_from_id}}, controller::{kafka::create_consumer, opensearch::{create, IndexDocument, delete}}};
use dotenv::dotenv;
use rdkafka::{Message, consumer::{CommitMode, Consumer}};
use salvo::{prelude::*, cors::Cors, hyper::Method};
extern crate serde_json;
use simple_logger::SimpleLogger;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new().env().init().unwrap();
    let host =  match env::var("HOST") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `HOST` Not found from enviroment, loaded from local IP");
            "127.0.0.1:7878".to_owned()
        }.to_owned(),
    };
    let kafka_enabled =  match env::var("KAFKA_ENABLED") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `KAFKA_ENABLED` Not found from enviroment, as default Y");
            "Y".to_owned()
        }.to_owned(),
    };
    let allowed_origin =  match env::var("ALLOWED_ORIGIN") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `ALLOWED_ORIGIN` Not found from enviroment");
            "*".to_owned()
        }.to_owned(),
    };
    //  Send Device Info
    log::info!("Server Address: {:?}", host.clone());
    let cors_handler = Cors::new()
    .allow_origin(&allowed_origin.to_owned())
    .allow_methods(vec![Method::GET, Method::POST, Method::DELETE]).into_handler();
    let router = Router::new()
        .hoop(cors_handler)
        .push(
            Router::with_path("v1/menus")
                .get(get_menu)
        )
        .push(
            Router::with_path("v1/process/<id>")
                .get(get_process)
        )
        .push(
            Router::with_path("v1/process")
                .get(get_process)
        )
        .push(
            Router::with_path("v1/browsers/<id>")
                .get(get_browsers)
        )
        .push(
            Router::with_path("v1/browsers")
                .get(get_browsers)
        )
        .push(
            Router::with_path("v1/windows/<id>")
                .get(get_windows)
        )
        .push(
            Router::with_path("v1/windows")
                .get(get_windows)
        )
        ;
    log::info!("{:#?}", router);
    let acceptor = TcpListener::new(&host).bind().await;
    let mut futures = vec![tokio::spawn(async move { Server::new(acceptor).serve(router).await; })];
    if kafka_enabled.eq("Y") {
        log::info!("Kafka Consumer is enabled");
        futures.push(tokio::spawn(async move { consume_queue().await; }));
    } else {
        log::info!("Kafka Consumer is disabled");
    }
    join_all(futures).await;
}

#[handler]
async fn get_menu<'a>(_req: &mut Request, _res: &mut Response) {
    let _id = _req.param::<i32>("id");
    if _id.is_some() {
        match menu_from_id(_id).await {
            Ok(menu) => _res.render(Json(menu)),
            Err(error) => _res.render(Json(error))
        }
    } else {
        let _language = _req.queries().get("language");
        let _client_id = _req.queries().get("client_id");
        let _role_id = _req.queries().get("role_id");
        let _user_id = _req.queries().get("user_id");
        let _search_value = _req.queries().get("search_value");
        match menus(_language, _client_id, _role_id, _user_id, _search_value).await {
            Ok(menu) => _res.render(Json(menu)),
            Err(e) => {
                _res.render(e.to_string());
                _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
            }
        }
    }
}

#[handler]
async fn get_process<'a>(_req: &mut Request, _res: &mut Response) {
    let _id = _req.param::<i32>("id");
    let _language = _req.queries().get("language");
    let _client_id = _req.queries().get("client_id");
    let _role_id = _req.queries().get("role_id");
    let _user_id = _req.queries().get("user_id");
    let _search_value = _req.queries().get("search_value");
    if _id.is_some() {
        match process_from_id(_language, _client_id, _role_id, _user_id, _id).await {
            Ok(process) => _res.render(Json(process)),
            Err(error) => _res.render(Json(error))
        }
    } else {
        match processes(_language, _client_id, _role_id, _user_id, _search_value).await {
            Ok(menu) => _res.render(Json(menu)),
            Err(e) => {
                _res.render(e.to_string());
                _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
            }
        }
    }
}

#[handler]
async fn get_browsers<'a>(_req: &mut Request, _res: &mut Response) {
    let _id = _req.param::<i32>("id");
    let _language = _req.queries().get("language");
    let _client_id = _req.queries().get("client_id");
    let _role_id = _req.queries().get("role_id");
    let _user_id = _req.queries().get("user_id");
    let _search_value = _req.queries().get("search_value");
    if _id.is_some() {
        match browser_from_id(_language, _client_id, _role_id, _user_id, _id).await {
            Ok(browser) => _res.render(Json(browser)),
            Err(error) => _res.render(Json(error))
        }
    } else {
        match browsers(_language, _client_id, _role_id, _user_id, _search_value).await {
            Ok(menu) => _res.render(Json(menu)),
            Err(e) => {
                _res.render(e.to_string());
                _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
            }
        }
    }
}

#[handler]
async fn get_windows<'a>(_req: &mut Request, _res: &mut Response) {
    let _id = _req.param::<i32>("id");
    let _language = _req.queries().get("language");
    let _client_id = _req.queries().get("client_id");
    let _role_id = _req.queries().get("role_id");
    let _user_id = _req.queries().get("user_id");
    let _search_value = _req.queries().get("search_value");
    if _id.is_some() {
        match window_from_id(_language, _client_id, _role_id, _user_id, _id).await {
            Ok(window) => _res.render(Json(window)),
            Err(error) => _res.render(Json(error))
        }
    } else {
        match windows(_language, _client_id, _role_id, _user_id, _search_value).await {
            Ok(menu) => _res.render(Json(menu)),
            Err(e) => {
                _res.render(e.to_string());
                _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
            }
        }
    }
}

async fn consume_queue() {
    let kafka_host =  match env::var("KAFKA_HOST") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `KAFKA_HOST` Not found from enviroment, loaded from local IP");
            "127.0.0.1:9092".to_owned()
        }.to_owned(),
    };
    let kafka_group =  match env::var("KAFKA_GROUP") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `KAFKA_GROUP` Not found from enviroment, loaded with `default` value");
            "default".to_owned()
        }.to_owned(),
    };
    let kafka_queues =  match env::var("KAFKA_QUEUES") {
        Ok(value) => value.clone(),
        Err(_) => {
            log::info!("Variable `KAFKA_QUEUES` Not found from enviroment, loaded with `default` value");
            "ad_menu".to_owned()
        }.to_owned(),
    };
    
    let topics: Vec<&str> = kafka_queues.split_whitespace().collect();
    let consumer = create_consumer(&kafka_host, &kafka_group, &topics);
    loop {
        match consumer.recv().await {
            Err(e) => log::error!("Kafka error: {}", e),
            Ok(message) => {
                let payload = match message.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        log::info!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                let key = match message.key_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        log::info!("Error while deserializing message key: {:?}", e);
                        ""
                    }
                };
                let event_type = key.replace("\"", "");
                let topic = message.topic();
                if topic == "menu" {
                    let _document = match serde_json::from_str(payload) {
                        Ok(value) => value,
                        Err(error) => {
                            log::warn!("{}", error);
                            MenuDocument {
                                document: None
                            }
                        },
                    };
                    if _document.document.is_some() {
                        let _menu_document: &dyn IndexDocument = &(_document.document.unwrap());
                        match process_index(event_type, _menu_document).await {
                            Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                            Err(error) => log::warn!("{}", error)
                        }
                    }
                } else if topic == "process" {
                    let _document = match serde_json::from_str(payload) {
                        Ok(value) => value,
                        Err(error) => {
                            log::warn!("{}", error);
                            ProcessDocument {
                                document: None
                            }
                        },
                    };
                    if _document.document.is_some() {
                        let _process_document: &dyn IndexDocument = &(_document.document.unwrap());
                        match process_index(event_type, _process_document).await {
                            Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                            Err(error) => log::warn!("{}", error)
                        }
                    }
                } else if topic == "browser" {
                    let _document = match serde_json::from_str(payload) {
                        Ok(value) => value,
                        Err(error) => {
                            log::warn!("{}", error);
                            BrowserDocument {
                                document: None
                            }
                        },
                    };
                    if _document.document.is_some() {
                        let _browser_document: &dyn IndexDocument = &(_document.document.unwrap());
                        match process_index(event_type, _browser_document).await {
                            Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                            Err(error) => log::warn!("{}", error)
                        }
                    }
                } else if topic == "window" {
                    let _document = match serde_json::from_str(payload) {
                        Ok(value) => value,
                        Err(error) => {
                            log::warn!("{}", error);
                            WindowDocument {
                                document: None
                            }
                        },
                    };
                    if _document.document.is_some() {
                        let _window_document: &dyn IndexDocument = &(_document.document.unwrap());
                        match process_index(event_type, _window_document).await {
                            Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                            Err(error) => log::warn!("{}", error)
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
}

async fn process_index(_event_type: String, _document: &dyn IndexDocument) -> Result<bool, std::string::String> {
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