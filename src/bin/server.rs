use std::env;
use opensearch_gateway_rs::{models::menu::{MenuDocument, menu_from_id, menus}, controller::{kafka::create_consumer, opensearch::{create, IndexDocument, delete}}};
use dotenv::dotenv;
use rdkafka::{Message, consumer::{CommitMode, Consumer}};
use salvo::prelude::*;
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
    //  Send Device Info
    log::info!("Server Address: {:?}", host.clone());
    let router = Router::new()
        .push(
            Router::with_path("v1/menus")
                .get(get_menu)
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
                    let _document: MenuDocument = match serde_json::from_str(payload) {
                        Ok(value) => value,
                        Err(error) => {
                            log::warn!("{}", error);
                            MenuDocument {
                                menu: None
                            }
                        },
                    };
                    if _document.menu.is_some() {
                        let _menu_document: &dyn IndexDocument = &(_document.menu.unwrap());
                        if event_type.eq("new") {
                            match create(_menu_document).await {
                                Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                Err(error) => log::warn!("{}", error)
                            }   
                        } else if event_type.eq("update") {
                            match delete(_menu_document).await {
                                Ok(_) => {
                                    match create(_menu_document).await {
                                        Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                        Err(error) => log::warn!("{}", error)
                                    }
                                },
                                Err(error) => log::warn!("{}", error)
                            } 
                        } else if event_type.eq("delete") {
                            match delete(_menu_document).await {
                                Ok(_) => consumer.commit_message(&message, CommitMode::Async).unwrap(),
                                Err(error) => log::warn!("{}", error)
                            }
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
