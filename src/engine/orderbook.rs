
use super::orders::OrderRequest;
use super::assets;
use super::assets::Asset;
use super::sequence;


const MIN_SEQUENCE_ID: u64 = 1;
const MAX_SEQUENCE_ID: u64 = 1000;


pub struct Orderbook {
    trading_asset: Asset,
    price_asset: Asset,
    seq: sequence::TradeSequence,
}


// todo fix test!

/// Create new orderbook for pair of assets
///
/// # Examples
///
/// Basic usage:
/// ```
/// let mut orderbook = new_orderbook(assets::BTC, assets::USD);
/// let result = orderbook.process_order(OrderRequest::MarketOrder{  });
/// assert_eq!(orderbook)
/// ```
pub fn new_orderbook(trading_asset: Asset, price_asset: Asset) -> Orderbook {
    Orderbook {
        trading_asset,
        price_asset,
        seq: sequence::new_sequence_gen(MIN_SEQUENCE_ID, MAX_SEQUENCE_ID),
    }
}

pub enum OrderProcessingResult {
    Accepted(u64),
    Rejected(String),
    Filled,
    Amended,
    Cancelled,
}


impl Orderbook {
    pub fn process_order(&mut self, order: OrderRequest) -> OrderProcessingResult {

        // todo processing logic

        let order_id = self.seq.next_id();
        OrderProcessingResult::Accepted(order_id)
    }

    // todo need it?
    pub fn get_trading_pair(&self) -> (Asset, Asset) {
        (self.trading_asset.clone(), self.price_asset.clone())
    }
}
