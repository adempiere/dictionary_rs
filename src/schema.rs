diesel::table! {
    c_orderline (c_orderline_id) {
        c_orderline_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
        processed -> Nullable<Text>,
    }
}

diesel::table! {
    c_order (c_order_id) {
        c_order_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
        processed -> Nullable<Text>,
    }
}

diesel::table! {
    c_bpartner (c_bpartner_id) {
        c_bpartner_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    c_bpartner_location (c_bpartner_location_id) {
        c_bpartner_location_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    m_pricelist (m_pricelist_id) {
        m_pricelist_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    m_product (m_product_id) {
        m_product_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    c_charge (c_charge_id) {
        c_charge_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    c_currency (c_currency_id) {
        c_currency_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    sfm_visitschedule (sfm_visitschedule_id) {
        sfm_visitschedule_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}

diesel::table! {
    sfm_visitscheduleline (sfm_visitscheduleline_id) {
        sfm_visitscheduleline_id -> Nullable<Numeric>,
        uuid -> Nullable<Text>,
    }
}