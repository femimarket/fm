pub mod summary;

use std::cell::OnceCell;
use serde::{Deserialize, Serialize};
use crate::instrument::Instrument;

const OANDA_SECRET: OnceCell<String> = OnceCell::new();

pub fn oanda_secret() -> String {
    OANDA_SECRET.get_or_init(|| {
        std::env::var("OANDA_SECRET").unwrap()
    }).to_string()
}

#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
pub struct Trade {
    pub price: i64,
    pub dt: i64,
    pub instrument: Instrument,
    pub qty: i64,
    // pub order_type: OrderType,
    // pub stop_price: i64,
    // pub limit_price: i64,
    // pub trigger: TriggerType,
    // pub status: StatusType,
    // pub pips: i64,
    // pub parent_id:i64,
    // pub pl:i64,
    // pub exit_price:i64,
}

pub fn backtest(trades:Vec<Trade>)->i64{
    let mut balance = 0i64;
    for (i,trade) in trades.iter().enumerate() {
        if i == 0 { continue }
        balance += (trade.price-trades[i-1].price)*trade.qty;
    }
    balance
}


#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub struct Summary {
    pub balance:f64,
    pub pl:f64
}