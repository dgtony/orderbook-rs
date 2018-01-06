
//use super::assets;
//use super::assets::TradePair;

use std::time::SystemTime;

use super::domain;
use super::domain::{OrderSide, Asset};





pub enum OrderRequest {
    NewMarketOrder {
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        qty: f64,
        ts: SystemTime,
    },
    NewLimitOrder {
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    AmendOrder {
        id: u64,
        side: OrderSide,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    CancelOrder {
        id: u64,
        side: OrderSide,
        ts: SystemTime,
    },
}


pub enum RequestError<'a> {
    UnsupportedAsset(&'a str),

    //todo moar errors!
}

/* Constructors */

// FIXME simplify signature?
pub fn new_market_order_request<'a>(
    order_asset: &'a str,
    price_asset: &'a str,
    side: OrderSide,
    qty: f64,
    ts: SystemTime,
) -> Result<OrderRequest, RequestError<'a>> {

    // some validation
    let order_asset = domain::parse_asset(order_asset).ok_or(
        RequestError::UnsupportedAsset(order_asset),
    )?;
    let price_asset = domain::parse_asset(price_asset).ok_or(
        RequestError::UnsupportedAsset(price_asset),
    )?;

    Ok(OrderRequest::NewMarketOrder {
        order_asset,
        price_asset,
        qty,
        side,
        ts,
    })
}


// TODO two more constructors!


pub fn limit_order_cancel_request(order_id: u64, side: OrderSide, ts: SystemTime) -> OrderRequest {
    OrderRequest::CancelOrder {
        id: order_id,
        side,
        ts,
    }
}
