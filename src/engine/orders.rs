
use std::time::SystemTime;
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
        //ts: SystemTime,
    },
}


/* Constructors */


/// Create request for the new market order
pub fn new_market_order_request(
    order_asset: Asset,
    price_asset: Asset,
    side: OrderSide,
    qty: f64,
    ts: SystemTime,
) -> OrderRequest {

    OrderRequest::NewMarketOrder {
        order_asset,
        price_asset,
        qty,
        side,
        ts,
    }
}


/// Create request for the new limit order
pub fn new_limit_order_request(
    order_asset: Asset,
    price_asset: Asset,
    side: OrderSide,
    price: f64,
    qty: f64,
    ts: SystemTime,
) -> OrderRequest {

    OrderRequest::NewLimitOrder {
        order_asset,
        price_asset,
        side,
        price,
        qty,
        ts,
    }
}


/// Create request for changing price/qty for the active limit order.
///
/// Note: do not change order side!
/// Instead cancel existing order and create a new one.
pub fn amend_order_request(
    id: u64,
    side: OrderSide,
    price: f64,
    qty: f64,
    ts: SystemTime,
) -> OrderRequest {

    OrderRequest::AmendOrder {
        id,
        side,
        price,
        qty,
        ts,
    }
}


/// Create request for cancelling active limit order
pub fn limit_order_cancel_request(order_id: u64, side: OrderSide) -> OrderRequest {
    OrderRequest::CancelOrder {
        id: order_id,
        side,
        //ts,
    }
}
