use std::{io::ErrorKind, io::Error, env};
use hyper::{Client, Body, Request, Response, body};
use salvo::prelude::*;//, hyper::{Request, Client, Body, body, Response}};

use crate::models::{documents::{SalesOrderLine, EntityDocument, Order, Entity, OrderLine}, persistence::{customer_by_uuid, order_line_by_uuid, order_by_uuid, product_by_uuid, charge_by_uuid, customer_location_by_uuid, currency_by_uuid, price_list_by_uuid, visit_schedule_by_uuid, visit_schedule_line_by_uuid}};


pub async fn create_or_order_line(_order_line: Option<SalesOrderLine>) -> Result<OrderLine, std::io::Error> {
    //  Validate complete
    if _order_line.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Order Line body not found"));
    }
    let _order_line = _order_line.unwrap();
    if _order_line.order_line_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Order Line is Mandatory"));
    }
    if _order_line.order_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Order is Mandatory"));
    }
    let order_line_uuid = _order_line.order_line_uuid.unwrap();
    let order_uuid = _order_line.order_uuid.unwrap();
    if _order_line.product_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Product is Mandatory"));
    }
    if _order_line.customer_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Customer is mandatory"));
    }
    if _order_line.customer_location.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Customer Location is Mandatory"));
    }
    if _order_line.customer_bill_location_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Customer Bill Location is Mandatory"));
    }
    if _order_line.currency_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Currency is Mandatory"));
    }
    if _order_line.price_list_uuid.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Price List is Mandatory"));
    }
    if _order_line.document_no.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Docuement No is Mandatory"));
    }
    if _order_line.date_ordered.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Date Ordered is Mandatory"));
    }
    if _order_line.quantity_entered.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Quantity Entered is Mandatory"));
    }
    if _order_line.quantity_ordered.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Quantity Ordered is Mandatory"));
    }
    if _order_line.price_entered.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Price Entered is Mandatory"));
    }
    if _order_line.price_list.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Price List is Mandatory"));
    }
    let customer = match customer_by_uuid(_order_line.customer_uuid) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Customer Not Found"));
        }
    };
    let customer_location = match customer_location_by_uuid(_order_line.customer_location) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Customer Location Not Found"));
        }
    };
    let customer_bill_location = match customer_location_by_uuid(_order_line.customer_bill_location_uuid) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Customer Bill Location Not Found"));
        }
    };
    let order_line = match order_line_by_uuid(Some(order_line_uuid.to_owned())) {
        Some(value) => {
            if value.to_owned().is_processed() {
                return Err(Error::new(ErrorKind::InvalidData.into(), "Order Line is Processed"));
            } else {
                Some(value)
            }
        },
        None => None
    };
    let order = match order_by_uuid(Some(order_uuid.to_owned())) {
        Some(value) => {
            if value.to_owned().is_processed() {
                return Err(Error::new(ErrorKind::InvalidData.into(), "Order is Processed"));
            } else {
                Some(value)
            }
        },
        None => None
    };
    let product = match product_by_uuid(_order_line.product_uuid) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Product Not Found"));
        }
    };
    let charge = match charge_by_uuid(_order_line.charge_uuid) {
        Some(value) => Some(value),
        None => None
    };
    let currency = match currency_by_uuid(_order_line.currency_uuid) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Currency Not Found"));
        }
    };
    let price_list = match price_list_by_uuid(_order_line.price_list_uuid) {
        Some(value) => value,
        None => {
            return Err(Error::new(ErrorKind::InvalidData.into(), "Price List Not Found"));
        }
    };
    let visit_schedule = match visit_schedule_by_uuid(_order_line.visit_schedule_uuid) {
        Some(value) => Some(value),
        None => None
    };
    let visit_schedule_line = match visit_schedule_line_by_uuid(_order_line.visit_schedule_line_uuid) {
        Some(value) => Some(value),
        None => None
    };
    //  Create Sales Order if not exists
    let order_id = match order {
        Some(value) => value.id(),
        None => Some({
            let mut sales_order = Order::default();
            sales_order.currency_id = currency.id();
            sales_order.customer_bill_location_id = customer_bill_location.id();
            sales_order.customer_id = customer.id();
            sales_order.customer_location_id = customer_location.id();
            sales_order.description = _order_line.description;
            sales_order.document_no = _order_line.document_no;
            sales_order.document_reference = _order_line.document_reference;
            sales_order.price_list_id = price_list.id();
            sales_order.uuid = Some(order_uuid.to_owned());
            if visit_schedule.is_some() {
                sales_order.visit_schedule_id = visit_schedule.unwrap().id();
            }
            if visit_schedule_line.is_some() {
                sales_order.visit_schedule_line_id = visit_schedule_line.unwrap().id();
            }
            match create_sales_order(sales_order).await {
                Err(error) => {
                    log::info!("Error {:?}", error);
                    return Err(Error::new(ErrorKind::InvalidData.into(), error));
                },
                Ok(value) => {
                    log::info!("Ok {:?}", value);
                    match update_document_uuid("C_Order".to_owned(), order_uuid, value.id.unwrap()).await {
                        Err(error_value) => {
                            log::info!("Error {:?}", error_value);
                            return Err(Error::new(ErrorKind::InvalidData.into(), error_value));
                        },
                        Ok(order_id) => order_id
                    }
                }
            }
        })
    };
    //  Add Order Line
    let created_order_line = match order_line {
        Some(_) => None,
        None => {
            let mut sales_order_line = OrderLine::default();
            if charge.is_some() {
                sales_order_line.charge_id = charge.unwrap().id();
            }
            sales_order_line.order_id = order_id;
            sales_order_line.price_entered = _order_line.price_entered;
            sales_order_line.price_list = _order_line.price_list;
            sales_order_line.product_id = product.id();
            sales_order_line.quantity_entered = _order_line.quantity_entered;
            sales_order_line.quantity_ordered = _order_line.quantity_ordered;
            sales_order_line.uuid = Some(order_line_uuid.to_owned());
            match create_sales_order_line(sales_order_line).await {
                Err(error) => {
                    log::info!("Error {:?}", error);
                    return Err(Error::new(ErrorKind::InvalidData.into(), error));
                },
                Ok(order_line) => {
                    let id = match update_document_uuid("C_OrderLine".to_owned(), order_line_uuid, order_line.id.unwrap()).await {
                        Err(error_value) => {
                            log::info!("Error {:?}", error_value);
                            return Err(Error::new(ErrorKind::InvalidData.into(), error_value));
                        },
                        Ok(order_line_id) => Some(order_line_id)
                    };
                    if id.is_some() {
                        return Ok(order_line);
                    } else {
                        return Err(Error::new(ErrorKind::InvalidData.into(), "Error Order Line no Created"));
                    }
                }
            }
        }
    };
    match created_order_line {
        None => return Err(Error::new(ErrorKind::InvalidData.into(), "Error Order Line already Created")),
        Some(line) => Ok(line)
    }
}

async fn create_sales_order(sales_order: Order) -> Result<Order, String> {
    let _response = middleware_request(EntityDocument::from_sales_order(sales_order.to_owned()), "POST".to_owned()).await;
    return match _response {
        Ok(response) => {
            let created_sales_order = Order::from_entity(response);
            log::info!("Sales Order Line ID: {:?}", created_sales_order.id);
            Ok(created_sales_order)
        },
        Err(value) => {
            log::error!("{:?}", value);
            Err(value.to_string())
        }
    }
}

async fn update_document_uuid(table_name: String, uuid: String, id: i32) -> Result<i32, String> {
    let _response = middleware_request(EntityDocument::for_update(table_name, uuid, id), "PATCH".to_owned()).await;
    return match _response {
        Ok(response) => {
            log::info!("Document ID: {:?}", response.id);
            Ok(response.id.unwrap_or_default())
        },
        Err(value) => {
            Err(value.to_string())
        }
    }
}

async fn middleware_request(_entity: EntityDocument, _method: String) -> Result<Entity, String> {
    let _adempiere_endpoint = env::var("ADEMPIERE_BACKEND_ENDPOINT").unwrap();
    let _adempiere_authorization = env::var("ADEMPIERE_AUTHORIZATION").unwrap();
    let _entity_as_string = serde_json::to_string(&_entity).expect("Error parsing to json");
    let _request = Request::builder()
        .method(_method.as_bytes())
        .uri(_adempiere_endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", _adempiere_authorization)
        .body(Body::from(_entity_as_string));
    if _request.is_ok() {
        let client = Client::new();
        return match client.request(_request.unwrap()).await {
            Ok(response) => {
                log::info!("Request Status: {:?}", response.status());
                match response.status() {
                    StatusCode::OK => {
                        let created_entity = match convert_entity_from_response(response).await {
                            Err(error) => {
                                return Err(error);
                            },
                            Ok(value) => value
                        };
                        log::info!("Entity ID: {:?}", created_entity.id);
                        Ok(created_entity)
                    },
                    _ => {
                        let error_message = match convert_error_from_response(response).await {
                            Err(error) => error,
                            Ok(value) => value
                        };
                        log::warn!("Request Error: {:?}", error_message);
                        Err(error_message)
                    }
                }
            },
            Err(value) => {
                log::error!("{:?}", value);
                Err(value.to_string())
            }
        }
    }
    Err("Request error".to_string())
}

async fn create_sales_order_line(sales_order_line: OrderLine) -> Result<OrderLine, String> {
    let _response = middleware_request(EntityDocument::from_sales_order_line(sales_order_line.to_owned()), "POST".to_owned()).await;
    return match _response {
        Ok(response) => {
            let created_sales_order_line = OrderLine::from_entity(response);
            log::info!("Sales Order Line ID: {:?}", created_sales_order_line.id);
            Ok(created_sales_order_line)
        },
        Err(value) => {
            log::error!("{:?}", value);
            Err(value.to_string())
        }
    }
}

async fn convert_error_from_response(response: Response<Body>) -> Result<String, String> {
    let body_bytes = match body::to_bytes(response.into_body()).await {
        Ok(value) => value,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error).to_string())
    };
    let body: String = match serde_json::from_slice(&body_bytes) {
        Ok(value) => value,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error).to_string())
    };
    Ok(body)
}

async fn convert_entity_from_response(response: Response<Body>) -> Result<Entity, String> {
    let body_bytes = match body::to_bytes(response.into_body()).await {
        Ok(value) => value,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error).to_string())
    };
    let body: Entity = match serde_json::from_slice(&body_bytes) {
        Ok(value) => value,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error).to_string())
    };
    Ok(body)
}