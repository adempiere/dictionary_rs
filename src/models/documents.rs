use serde::{Deserialize, Serialize};
use salvo::prelude::*;
// use super::readers::Attendance;
extern crate diesel;

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct DocumentOrderLine {
    pub order_line: Option<SalesOrderLine>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct SalesOrderLine {
    pub order_line_uuid: Option<String>,
    pub order_uuid: Option<String>,
    pub product_uuid: Option<String>,
    pub charge_uuid: Option<String>,
    pub customer_uuid: Option<String>,
    pub customer_location: Option<String>,
    pub customer_bill_location_uuid: Option<String>,
    pub currency_uuid: Option<String>,
    pub price_list_uuid: Option<String>,
    pub visit_schedule_uuid: Option<String>,
    pub visit_schedule_line_uuid: Option<String>,
    pub document_no: Option<String>,
    pub date_ordered: Option<String>,
    pub description: Option<String>,
    pub document_reference: Option<String>,
    pub quantity_entered: Option<f64>,
    pub quantity_ordered: Option<f64>,
    pub price_entered: Option<f64>,
    pub price_list: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub document_no: Option<String>,
    pub document_reference: Option<String>,
    pub description: Option<String>,
    pub customer_id: Option<i32>,
    pub customer_location_id: Option<i32>,
    pub customer_bill_location_id: Option<i32>,
    pub price_list_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub visit_schedule_id: Option<i32>,
    pub visit_schedule_line_id: Option<i32>,
}

impl Order {
    pub fn from_entity(_source: Entity) -> Self {
        let mut order = Order::default();
        if _source.attributes.is_some() {
            order.id = _source.id;
            let attributes = _source.attributes.unwrap();
            order.currency_id = match attributes.iter().find(|&value| value.key.eq("C_Currency_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.customer_bill_location_id = match attributes.iter().find(|&value| value.key.eq("C_BPartner_Location_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.customer_id = match attributes.iter().find(|&value| value.key.eq("C_BPartner_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.customer_location_id = match attributes.iter().find(|&value| value.key.eq("C_BPartner_Location_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.description = match attributes.iter().find(|&value| value.key.eq("Description")) {
                Some(value) => value.to_owned().string_value,
                None => None
            };
            order.document_no = match attributes.iter().find(|&value| value.key.eq("DocumentNo")) {
                Some(value) => value.to_owned().string_value,
                None => None
            };
            order.document_reference = match attributes.iter().find(|&value| value.key.eq("POReference")) {
                Some(value) => value.to_owned().string_value,
                None => None
            };
            order.price_list_id = match attributes.iter().find(|&value| value.key.eq("M_PriceList_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.visit_schedule_id = match attributes.iter().find(|&value| value.key.eq("SFM_VisitSchedule_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.visit_schedule_line_id = match attributes.iter().find(|&value| value.key.eq("SFM_VisitScheduleLine_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order.uuid = match attributes.iter().find(|&value| value.key.eq("UUID")) {
                Some(value) => value.to_owned().string_value,
                None => None
            };
        }
        // sales_order
        order
    }
}

impl Default for Order {
    fn default() -> Self {
        Order {
            currency_id: None,
            customer_bill_location_id: None,
            customer_id: None,
            customer_location_id: None,
            description: None,
            document_no: None,
            document_reference: None,
            id: None,
            price_list_id: None,
            uuid: None,
            visit_schedule_id: None,
            visit_schedule_line_id: None
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderLine {
    pub id: Option<i32>,
    pub order_id: Option<i32>,
    pub uuid: Option<String>,
    pub product_id: Option<i32>,
    pub charge_id: Option<i32>,
    pub quantity_entered: Option<f64>,
    pub quantity_ordered: Option<f64>,
    pub price_entered: Option<f64>,
    pub price_list: Option<f64>,
}

impl OrderLine {
    pub fn from_entity(_source: Entity) -> Self {
        let mut order_line = OrderLine::default();
        if _source.attributes.is_some() {
            order_line.id = _source.id;
            let attributes = _source.attributes.unwrap();
            order_line.product_id = match attributes.iter().find(|&value| value.key.eq("M_Product_ID")) {
                Some(value) => value.integer_value,
                None => None
            };
            order_line.quantity_entered = match attributes.iter().find(|&value| value.key.eq("QtyEntered")) {
                Some(value) => value.decimal_value,
                None => None
            };
            order_line.quantity_ordered = match attributes.iter().find(|&value| value.key.eq("QtyOrdered")) {
                Some(value) => value.decimal_value,
                None => None
            };
            order_line.price_entered = match attributes.iter().find(|&value| value.key.eq("PriceEntered")) {
                Some(value) => value.decimal_value,
                None => None
            };
            order_line.price_list = match attributes.iter().find(|&value| value.key.eq("PriceList")) {
                Some(value) => value.decimal_value,
                None => None
            };
            order_line.uuid = match attributes.iter().find(|&value| value.key.eq("UUID")) {
                Some(value) => value.to_owned().string_value,
                None => None
            };
        }
        // sales_order
        order_line
    }
}

impl Default for OrderLine {
    fn default() -> Self {
        OrderLine {
            charge_id: None,
            id: None,
            order_id: None,
            price_entered: None,
            price_list: None,
            product_id: None,
            quantity_entered: None,
            quantity_ordered: None,
            uuid: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    pub id: Option<i32>,
    pub table_name: Option<String>,
    pub attributes: Option<Vec<KeyAndValue>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyAndValue {
    pub key: String,
    pub integer_value: Option<i32>,
    pub boolean_value: Option<bool>,
    pub string_value: Option<String>,
    pub date_value: Option<String>,
    pub decimal_value: Option<f64>,
    pub value_type: Option<String> 
}

impl Default for KeyAndValue {
    fn default() -> Self {
        KeyAndValue { 
            key: "".to_owned(), 
            integer_value: None, 
            boolean_value: None, 
            string_value: None, 
            date_value: None, 
            decimal_value: 
            None, 
            value_type: None 
        }
    }
}

impl KeyAndValue {
    pub fn with_key(mut self, _key: String) -> Self {
        self.key = _key;
        self
    }

    pub fn with_integer_value(mut self, _integer_value: i32) -> Self {
        self.integer_value = Some(_integer_value);
        self
    }

    pub fn with_string_value(mut self, _string_value: String) -> Self {
        self.string_value = Some(_string_value);
        self
    }

    pub fn with_date_value(mut self, _date_value: String) -> Self {
        self.date_value = Some(_date_value);
        self
    }

    pub fn with_boolean_value(mut self, _boolean_value: bool) -> Self {
        self.boolean_value = Some(_boolean_value);
        self
    }

    pub fn with_decimal_value(mut self, _decimal_value: f64) -> Self {
        self.decimal_value = Some(_decimal_value);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityDocument {
    pub entity: Option<Entity>
}

impl Default for EntityDocument {
    fn default() -> Self {
        Self { entity: Default::default() }
    }
}

impl EntityDocument {
    pub fn for_update(table_name: String, uuid: String, id: i32) -> Self {
        let mut attributes: Vec<KeyAndValue>= Vec::new();
        attributes.push(KeyAndValue::default().with_key("UUID".to_string()).with_string_value(uuid));
        EntityDocument { 
            entity: Some(Entity {
                id: Some(id),
                table_name: Some(table_name.to_string()),
                attributes: Some(attributes)
            })
        }
    }

    pub fn from_sales_order_line_only_id(_source: OrderLine) -> Self {
        let mut attributes: Vec<KeyAndValue>= Vec::new();
        attributes.push(KeyAndValue::default().with_key("UUID".to_string()).with_string_value(_source.uuid.unwrap()));
        EntityDocument { 
            entity: Some(Entity {
                id: _source.id,
                table_name: Some("C_OrderLine".to_string()),
                attributes: Some(attributes)
            })
        }
    }

    pub fn from_sales_order(_source: Order) -> Self {
        let mut attributes: Vec<KeyAndValue>= Vec::new();
        attributes.push(KeyAndValue::default().with_key("UUID".to_string()).with_string_value(_source.uuid.unwrap()));
        attributes.push(KeyAndValue::default().with_key("DocumentNo".to_string()).with_string_value(_source.document_no.unwrap()));
        attributes.push(KeyAndValue::default().with_key("POReference".to_string()).with_string_value(_source.document_reference.unwrap()));
        attributes.push(KeyAndValue::default().with_key("Description".to_string()).with_string_value(_source.description.unwrap()));
        attributes.push(KeyAndValue::default().with_key("C_BPartner_ID".to_string()).with_integer_value(_source.customer_id.unwrap()));
        attributes.push(KeyAndValue::default().with_key("C_BPartner_Location_ID".to_string()).with_integer_value(_source.customer_location_id.unwrap()));
        attributes.push(KeyAndValue::default().with_key("Bill_Location_ID".to_string()).with_integer_value(_source.customer_bill_location_id.unwrap()));
        attributes.push(KeyAndValue::default().with_key("M_PriceList_ID".to_string()).with_integer_value(_source.price_list_id.unwrap()));
        attributes.push(KeyAndValue::default().with_key("C_Currency_ID".to_string()).with_integer_value(_source.currency_id.unwrap()));
        if _source.visit_schedule_id.is_some() {
            attributes.push(KeyAndValue::default().with_key("SFM_VisitSchedule_ID".to_string()).with_integer_value(_source.visit_schedule_id.unwrap()));
        }
        if _source.visit_schedule_line_id.is_some() {
            attributes.push(KeyAndValue::default().with_key("SFM_VisitScheduleLine_ID".to_string()).with_integer_value(_source.visit_schedule_line_id.unwrap()));
        }
        // attributes.push(KeyAndValue::default().with_key("M_Warehouse_ID".to_string()).with_integer_value(1000027));
        attributes.push(KeyAndValue::default().with_key("C_DocTypeTarget_ID".to_string()).with_integer_value(1000162));
        // attributes.push(KeyAndValue::default().with_key("AttendanceTime".to_string()).with_date_value(_source.attendance_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string()));
        EntityDocument { 
            entity: Some(Entity {
                id: None,
                table_name: Some("C_Order".to_string()),
                attributes: Some(attributes)
            })
        }
    }

    pub fn from_sales_order_line(_source: OrderLine) -> Self {
        let mut attributes: Vec<KeyAndValue>= Vec::new();
        attributes.push(KeyAndValue::default().with_key("UUID".to_string()).with_string_value(_source.uuid.unwrap()));
        attributes.push(KeyAndValue::default().with_key("C_Order_ID".to_string()).with_integer_value(_source.order_id.unwrap()));
        attributes.push(KeyAndValue::default().with_key("M_Product_ID".to_string()).with_integer_value(_source.product_id.unwrap()));
        if _source.charge_id.is_some() {
            attributes.push(KeyAndValue::default().with_key("C_Charge_ID".to_string()).with_integer_value(_source.charge_id.unwrap()));
        }
        attributes.push(KeyAndValue::default().with_key("QtyEntered".to_string()).with_decimal_value(_source.quantity_entered.unwrap()));
        attributes.push(KeyAndValue::default().with_key("QtyOrdered".to_string()).with_decimal_value(_source.quantity_ordered.unwrap()));
        attributes.push(KeyAndValue::default().with_key("PriceEntered".to_string()).with_decimal_value(_source.price_entered.unwrap()));
        attributes.push(KeyAndValue::default().with_key("PriceList".to_string()).with_decimal_value(_source.price_list.unwrap()));
        EntityDocument { 
            entity: Some(Entity {
                id: None,
                table_name: Some("C_OrderLine".to_string()),
                attributes: Some(attributes)
            })
        }
    }
}