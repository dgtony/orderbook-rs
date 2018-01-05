

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Asset {
    USD,
    EUR,
    BTC,
    ETH,
    OTN,
}


// fixme remove?
pub struct TradePair {
    pub from: Asset,
    pub to: Asset,
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
