use std::env;
use opensearch_gateway_rs::{models::menu::MenuDocument, controller::{kafka::create_consumer, opensearch::{create, IndexDocument, delete}}};
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
    //  Send Device Info
    log::info!("Server Address: {:?}", host.clone());
    let router = Router::new()
        // .push(
        //     Router::with_path("v1/order_lines")
        //         .post(create_order_line)
        // )
        ;
    log::info!("{:#?}", router);
    let acceptor = TcpListener::new(&host).bind().await;
    let futures = vec![
                tokio::spawn(async move { consume_queue().await }), 
                tokio::spawn(async move { Server::new(acceptor).serve(router).await; })];
    join_all(futures).await;
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
                if topic == "ad_menu" {
                    let _document: MenuDocument = serde_json::from_str(payload).expect("Error with payload");
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

// #[handler]
// async fn create_order_line<'a>(_document: MenuDocument, _res: &mut Response) {
//     log::info!("create_order_line call");
//     let _order_line = _document.order_line;
//     if _order_line.is_none() {
//         _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
//     } else {
//         match create_or_order_line(_order_line).await {
//             Ok(value) => {
//                 _res.render(Json(value));
//                 _res.status_code(StatusCode::OK);    
//             },
//             Err(e) => {
//                 _res.render(e.to_string());
//                 _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
//             }
//         }
//     }
//     log::info!("create_order_line called");
// }
