
mod engine;

pub use engine::domain;
pub use engine::orderbook::Orderbook;
pub use engine::orders;




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn try() {
        use std::time::SystemTime;

        let mut orderbook = Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_market_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            2.0,
            SystemTime::now(),
        );

        let res = orderbook.process_order(order1);

        println!("result: {:?}", res);
        assert!(false)
    }
}
