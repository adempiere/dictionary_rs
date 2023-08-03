use diesel::prelude::*;
use cached::proc_macro::once;

use crate::{schema::{self, c_bpartner, c_order, c_orderline, m_product, m_pricelist, c_currency, sfm_visitschedule, sfm_visitscheduleline, c_bpartner_location, c_charge}, establish_connection, models::readers::DocumentReference};

use super::readers::EntityReference;

#[once(time=10, option = true, sync_writes = true)]
pub fn customer_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call customer without cache");
    let _connection = &mut establish_connection();
    let _result = c_bpartner::table
        .filter(schema::c_bpartner::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("customer_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn customer_location_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call customer location without cache");
    let _connection = &mut establish_connection();
    let _result = c_bpartner_location::table
        .filter(schema::c_bpartner_location::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("customer_location_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

pub fn order_by_uuid(uuid: Option<String>) -> Option<DocumentReference>{
    log::info!("call order without cache");
    let _connection = &mut establish_connection();
    let _result = c_order::table
        .filter(schema::c_order::uuid.eq(uuid))
        .first::<DocumentReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("order_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

pub fn order_line_by_uuid(uuid: Option<String>) -> Option<DocumentReference>{
    log::info!("call order line without cache");
    let _connection = &mut establish_connection();
    let _result = c_orderline::table
        .filter(schema::c_orderline::uuid.eq(uuid))
        .first::<DocumentReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("order_line_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn product_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call product without cache");
    let _connection = &mut establish_connection();
    let _result = m_product::table
        .filter(schema::m_product::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("product_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn charge_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call charge without cache");
    let _connection = &mut establish_connection();
    let _result = c_charge::table
        .filter(schema::c_charge::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("charge_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn price_list_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call price list without cache");
    let _connection = &mut establish_connection();
    let _result = m_pricelist::table
        .filter(schema::m_pricelist::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("price_list_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn currency_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call currency without cache");
    let _connection = &mut establish_connection();
    let _result = c_currency::table
        .filter(schema::c_currency::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("currency_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn visit_schedule_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call visit schedule without cache");
    let _connection = &mut establish_connection();
    let _result = sfm_visitschedule::table
        .filter(schema::sfm_visitschedule::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("visit_schedule_by_uuid: {:?}", error.to_string());
            None
        }
    }
}

#[once(time=10, option = true, sync_writes = true)]
pub fn visit_schedule_line_by_uuid(uuid: Option<String>) -> Option<EntityReference>{
    log::info!("call visit schedule line without cache");
    let _connection = &mut establish_connection();
    let _result = sfm_visitscheduleline::table
        .filter(schema::sfm_visitscheduleline::uuid.eq(uuid))
        .first::<EntityReference>(_connection);
    match _result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("visit_schedule_line_by_uuid: {:?}", error.to_string());
            None
        }
    }
}