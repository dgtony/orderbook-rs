
use super::domain::Asset;
use super::orders::OrderRequest;


/// Validation errors
const ERR_BAD_ORDER_ASSET: &str = "bad order asset";
const ERR_BAD_PRICE_ASSET: &str = "bad price asset";
const ERR_BAD_PRICE_VALUE: &str = "price must be non-negative";
const ERR_BAD_QUANTITY_VALUE: &str = "quantity must be non-negative";
const ERR_BAD_SEQ_ID: &str = "order ID out of range";


/* Validators */

pub struct OrderRequestValidator {
    orderbook_order_asset: Asset,
    orderbook_price_asset: Asset,
    min_sequence_id: u64,
    max_sequence_id: u64,
}

impl OrderRequestValidator {
    pub fn new(
        orderbook_order_asset: Asset,
        orderbook_price_asset: Asset,
        min_sequence_id: u64,
        max_sequence_id: u64,
    ) -> Self {
        OrderRequestValidator {
            orderbook_order_asset,
            orderbook_price_asset,
            min_sequence_id,
            max_sequence_id,
        }
    }

    pub fn validate(&self, request: &OrderRequest) -> Result<(), &str> {
        match *request {
            OrderRequest::MarketOrder {
                order_asset,
                price_asset,
                side,
                qty,
                ts,
            } => self.validate_market(order_asset, price_asset, qty),

            OrderRequest::LimitOrder {
                order_asset,
                price_asset,
                side,
                price,
                qty,
                ts,
            } => self.validate_limit(order_asset, price_asset, price, qty),

            OrderRequest::AmendOrder {
                id,
                price,
                side,
                qty,
                ts,
            } => self.validate_amend(id, price, qty),

            OrderRequest::CancelOrder { id, side, ts } => self.validate_cancel(id),
        }
    }

    /* Internal validators */

    fn validate_market(
        &self,
        order_asset: Asset,
        price_asset: Asset,
        qty: f64,
    ) -> Result<(), &str> {

        if self.orderbook_order_asset != order_asset {
            return Err(ERR_BAD_ORDER_ASSET);
        }

        if self.orderbook_price_asset != price_asset {
            return Err(ERR_BAD_PRICE_ASSET);
        }

        if qty <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }


    fn validate_limit(
        &self,
        order_asset: Asset,
        price_asset: Asset,
        price: f64,
        qty: f64,
    ) -> Result<(), &str> {

        if self.orderbook_order_asset != order_asset {
            return Err(ERR_BAD_ORDER_ASSET);
        }

        if self.orderbook_price_asset != price_asset {
            return Err(ERR_BAD_PRICE_ASSET);
        }

        if price <= 0.0 {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if qty <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }

    fn validate_amend(&self, id: u64, price: f64, qty: f64) -> Result<(), &str> {
        if self.min_sequence_id > id || self.max_sequence_id < id {
            return Err(ERR_BAD_SEQ_ID);
        }

        if price <= 0.0 {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if qty <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }

    fn validate_cancel(&self, id: u64) -> Result<(), &str> {
        if self.min_sequence_id > id || self.max_sequence_id < id {
            return Err(ERR_BAD_SEQ_ID);
        }

        Ok(())
    }
}


// FIXME ?
// Better obtain this info from admin DB
pub fn can_trade(asset_from: &Asset, asset_to: &Asset) -> bool {
    match (asset_from, asset_to) {
        (_, &Asset::USD) => true,
        (_, &Asset::EUR) => true,
        (_, &Asset::BTC) => true,
        (&Asset::BTC, &Asset::ETH) => true,
        (&Asset::BTC, &Asset::OTN) => true,
        _ => false,
    }
}
