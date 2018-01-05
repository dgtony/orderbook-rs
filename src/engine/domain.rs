

#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Bid,
    Ask,
}


pub struct Order {
    pub order_asset: Asset,
    pub price_asset: Asset,
    pub price: f64,
    pub qty: f64,
    pub side: OrderSide,
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
