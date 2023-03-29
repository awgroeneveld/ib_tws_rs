//use bytes::{Buf, BufMut, Bytes, BytesMut};
//use encoder::buf::TwsEncoder;

use std::convert::From;
// PriceConditionTriggerMode

pub const PTM_DEFAULT: i32 = 0;
pub const PTM_DOUBLE_ASK_BID: i32 = 1;
pub const PTM_LAST: i32 = 2;
pub const PTM_DOUBLE_LAST: i32 = 3;
pub const PTM_BID_ASK: i32 = 4;
pub const PTM_LAST_OF_BID_ASK: i32 = 7;
pub const PTM_MID_POINT: i32 = 8;

#[derive(Debug, Clone)]
pub struct Price {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub contract_id: i32,
    pub exchange: String,
    pub price: f64,
    pub trigger_mode: i32,
}

#[derive(Debug, Clone)]
pub struct Time {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub time: String,
}

#[derive(Debug, Clone)]
pub struct Margin {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub percent: i32,
}

#[derive(Debug, Clone)]
pub struct Execution {
    // inherit orderCondition
    pub is_conjunction_connection: bool,
    pub sec_type: String,
    pub exchange: String,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct Volume {
    // inherit ContractCondition
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub conid: i32,
    pub exchange: String,
    pub volume: i32,
}

#[derive(Debug, Clone)]
pub struct PercentChange {
    // inherit ContractCondition
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub conid: i32,
    pub exchange: String,
    pub change_percent: f64,
}

#[derive(Debug, Clone)]
pub enum OrderCondition {
    PriceCondition(Price),
    TimeCondition(Time),
    MarginCondition(Margin),
    ExecutionCondition(Execution),
    VolumeCondition(Volume),
    PercentChangeCondition(PercentChange),
}

impl OrderCondition {
    #[must_use]
    pub fn type_val(&self) -> i32 {
        match self {
            OrderCondition::PriceCondition(_) => 1,
            OrderCondition::TimeCondition(_) => 3,
            OrderCondition::MarginCondition(_) => 4,
            OrderCondition::ExecutionCondition(_) => 5,
            OrderCondition::VolumeCondition(_) => 6,
            OrderCondition::PercentChangeCondition(_) => 7,
        }
    }
}

impl From<OrderCondition> for i32 {
    fn from(condition: OrderCondition) -> i32 {
        match condition {
            OrderCondition::PriceCondition(_) => 1,
            OrderCondition::TimeCondition(_) => 3,
            OrderCondition::MarginCondition(_) => 4,
            OrderCondition::ExecutionCondition(_) => 5,
            OrderCondition::VolumeCondition(_) => 6,
            OrderCondition::PercentChangeCondition(_) => 7,
        }
    }
}
