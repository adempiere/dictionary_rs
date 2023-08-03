use std::env;
use microservice_postgresql_rs::{models::documents::DocumentOrderLine, controller::create_or_order_line};
use dotenv::dotenv;
use salvo::prelude::*;
extern crate serde_json;
use simple_logger::SimpleLogger;
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
        .push(
            Router::with_path("v1/order_lines")
                .post(create_order_line)
        )
        ;
    log::info!("{:#?}", router);
    let acceptor = TcpListener::new(&host).bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn create_order_line<'a>(_document: DocumentOrderLine, _res: &mut Response) {
    log::info!("create_order_line call");
    let _order_line = _document.order_line;
    if _order_line.is_none() {
        _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
    } else {
        match create_or_order_line(_order_line).await {
            Ok(value) => {
                _res.render(Json(value));
                _res.status_code(StatusCode::OK);    
            },
            Err(e) => {
                _res.render(e.to_string());
                _res.status_code(StatusCode::INTERNAL_SERVER_ERROR);    
            }
        }
    }
    log::info!("create_order_line called");
}
