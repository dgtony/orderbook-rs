
use super::assets;
use super::assets::TradePair;
use super::order_queues::OrderSide;



pub enum OrderRequest {
    MarketOrder {
        pair: TradePair, // fixme!
        qty: f64,
        side: OrderSide,
    },
    LimitOrder {
        pair: TradePair,
        price: f64,
        qty: f64,
        side: OrderSide,
    },
    AmendOrder { id: u64, price: f64, qty: f64 },
    CancelOrder { id: u64 },
}


pub enum OrderError<'a> {
    UnsupportedAsset(&'a str),
    NotTradingPair,
}


pub fn new_market_order<'a>(
    asset_from: &'a str,
    asset_to: &'a str,
    side: OrderSide,
    qty: f64,
) -> Result<OrderRequest, OrderError<'a>> {
    // todo: get id sequence
    let order_id: u64 = 1;

    // some validation
    let asset_from = assets::parse_asset(asset_from).ok_or(
        OrderError::UnsupportedAsset(asset_from),
    )?;
    let asset_to = assets::parse_asset(asset_to).ok_or(
        OrderError::UnsupportedAsset(
            asset_to,
        ),
    )?;

    if !assets::can_trade(&asset_from, &asset_to) {
        return Err(OrderError::NotTradingPair);
    }

    Ok(OrderRequest::MarketOrder {
        pair: TradePair {
            from: asset_from,
            to: asset_to,
        },
        qty,
        side,
    })
}
