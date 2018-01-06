

#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u64,
    pub order_asset: Asset,
    pub price_asset: Asset,
    pub side: OrderSide,
    pub price: f64,
    pub qty: f64,
}


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
}


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Asset {
    USD,
    EUR,
    BTC,
    ETH,
    OTN,
}


pub fn parse_asset(asset: &str) -> Option<Asset> {
    match asset {
        "USD" => Some(Asset::USD),
        "EUR" => Some(Asset::EUR),
        "BTC" => Some(Asset::BTC),
        "ETH" => Some(Asset::ETH),
        "OTN" => Some(Asset::OTN),
        _ => None,
    }
}
