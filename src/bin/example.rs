
extern crate orderbook;

use std::time::SystemTime;
use orderbook::{Orderbook, OrderSide, orders};


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SupportedAssets {
    USD,
    EUR,
    BTC,
    ETH,
    OTN,
}


fn parse_asset(asset: &str) -> Option<SupportedAssets> {
    match asset {
        "USD" => Some(SupportedAssets::USD),
        "EUR" => Some(SupportedAssets::EUR),
        "BTC" => Some(SupportedAssets::BTC),
        "ETH" => Some(SupportedAssets::ETH),
        "OTN" => Some(SupportedAssets::OTN),
        _ => None,
    }
}


fn main() {
    let mut orderbook = Orderbook::new(SupportedAssets::BTC, SupportedAssets::USD);
    let order_asset = parse_asset("BTC").unwrap();
    let price_asset = parse_asset("USD").unwrap();

    let order_list = vec![
        orders::new_limit_order_request(order_asset, price_asset, OrderSide::Ask, 1.02, 1.0, SystemTime::now()),
        orders::new_limit_order_request(order_asset, price_asset, OrderSide::Ask, 1.03, 0.5, SystemTime::now()),
        orders::new_limit_order_request(order_asset, price_asset, OrderSide::Ask, 1.05, 0.5, SystemTime::now()),
        orders::new_limit_order_request(order_asset, price_asset, OrderSide::Bid, 1.06, 2.0, SystemTime::now()),
    ];

    for order in order_list {
        println!("processing order: {:?}", &order);
        let res = orderbook.process_order(order);
        println!("result: {:?}", res);
    }
}