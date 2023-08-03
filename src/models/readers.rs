use diesel::Queryable;
use bigdecimal::{BigDecimal, ToPrimitive};

#[derive(Queryable, Debug, Clone)]
pub struct EntityReference {
    pub id: Option<BigDecimal>,
    pub uuid: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct DocumentReference {
    pub id: Option<BigDecimal>,
    pub uuid: Option<String>,
    pub processed: Option<String>,
}

impl DocumentReference {
    pub fn is_processed(self) -> bool {
        match self.processed {
            Some(value) => {
                value.eq("Y")
            },
            None => false
        }
    }

    pub fn id(self) -> Option<i32> {
        match self.id {
            Some(value) => {
                value.to_i32()
            },
            None => None
        }
    }
}

impl EntityReference {
    pub fn id(self) -> Option<i32> {
        match self.id {
            Some(value) => {
                value.to_i32()
            },
            None => None
        }
    }
}