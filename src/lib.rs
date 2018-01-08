
mod engine;

pub use engine::domain;
pub use engine::orderbook;
pub use engine::orders;


#[cfg(test)]
mod tests {
    use super::*;
    use super::orderbook::{Success, Failed};

    const FLOAT_THRESHOLD: f64 = 1e-6;

    fn match_float(expected: f64, get: f64) -> bool {
        if (expected - get).abs() < FLOAT_THRESHOLD {
            true
        } else {
            false
        }
    }


    #[test]
    fn market_order_on_empty_orderbook() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_market_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            2.0,
            SystemTime::now(),
        );

        // process market order
        let res = orderbook.process_order(order1);

        if !match res[0] {
            Ok(Success::Accepted { id: 1, .. }) => true,
            _ => false,
        } ||
            !match res[1] {
                Err(Failed::NoMatch(1)) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res)
        }
    }


    #[test]
    fn market_order_partial_match() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            1.0,
            SystemTime::now(),
        );

        let order2 = orders::new_market_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            0.5,
            SystemTime::now(),
        );

        orderbook.process_order(order1);
        let res = orderbook.process_order(order2);

        if !match res[0] {
            Ok(Success::Accepted { id: 2, .. }) => true,
            _ => false,
        } ||
            !match res[1] {
                Ok(Success::Filled {
                       order_id: 2,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            } ||
            !match res[2] {
                Ok(Success::PartiallyFilled {
                       order_id: 1,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res)
        }
    }


    #[test]
    fn market_order_two_orders_match() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            1.0,
            SystemTime::now(),
        );

        let order2 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            12.0,
            1.0,
            SystemTime::now(),
        );

        let order3 = orders::new_market_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            1.5,
            SystemTime::now(),
        );

        orderbook.process_order(order1);
        orderbook.process_order(order2);
        let res = orderbook.process_order(order3);

        if !match res[0] {
            Ok(Success::Accepted { id: 3, .. }) => true,
            _ => false,
        } ||
            !match res[1] {
                Ok(Success::PartiallyFilled {
                       order_id: 3,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 12.0) && match_float(qty, 1.0) => true,
                _ => false,
            } ||
            !match res[2] {
                Ok(Success::Filled {
                       order_id: 2,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 12.0) && match_float(qty, 1.0) => true,
                _ => false,
            } ||
            !match res[3] {
                Ok(Success::Filled {
                       order_id: 3,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            } ||
            !match res[4] {
                Ok(Success::PartiallyFilled {
                       order_id: 1,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res)
        }
    }


    #[test]
    fn limit_order_on_empty_orderbook() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            2.0,
            SystemTime::now(),
        );

        // process order
        let res = orderbook.process_order(order1);

        if !match res[0] {
            Ok(Success::Accepted { id: 1, .. }) => true,
            _ => false,
        }
        {
            panic!("unexpected event sequence: {:?}", res)
        }
    }


    #[test]
    fn limit_order_partial_match() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            1.0,
            SystemTime::now(),
        );

        let order2 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            9.0,
            0.5,
            SystemTime::now(),
        );

        orderbook.process_order(order1);
        let res = orderbook.process_order(order2);

        if !match res[0] {
            Ok(Success::Accepted { id: 2, .. }) => true,
            _ => false,
        } ||
            !match res[1] {
                Ok(Success::Filled {
                       order_id: 2,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            } ||
            !match res[2] {
                Ok(Success::PartiallyFilled {
                       order_id: 1,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res)
        }
    }


    #[test]
    fn limit_order_exact_match() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            1.0,
            SystemTime::now(),
        );

        let order2 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            9.0,
            0.5,
            SystemTime::now(),
        );

        orderbook.process_order(order1);
        let res = orderbook.process_order(order2);

        if !match res[0] {
            Ok(Success::Accepted { id: 2, .. }) => true,
            _ => false,
        } ||
            !match res[1] {
                Ok(Success::Filled {
                       order_id: 2,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            } ||
            !match res[2] {
                Ok(Success::PartiallyFilled {
                       order_id: 1,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res)
        }

        let order3 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            8.0,
            0.5,
            SystemTime::now(),
        );

        let res2 = orderbook.process_order(order3);

        if !match res2[0] {
            Ok(Success::Accepted { id: 3, .. }) => true,
            _ => false,
        } ||
            !match res2[1] {
                Ok(Success::Filled {
                       order_id: 3,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            } ||
            !match res2[2] {
                Ok(Success::Filled {
                       order_id: 1,
                       price,
                       qty,
                       ..
                   }) if match_float(price, 10.0) && match_float(qty, 0.5) => true,
                _ => false,
            }
        {
            panic!("unexpected event sequence: {:?}", res2)
        }

        assert_eq!(orderbook.current_spread(), None);
    }


    #[test]
    fn current_spread() {
        use std::time::SystemTime;

        let mut orderbook = orderbook::Orderbook::new(domain::Asset::BTC, domain::Asset::USD);
        let order_asset = domain::parse_asset("BTC").unwrap();
        let price_asset = domain::parse_asset("USD").unwrap();

        let order1 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            10.0,
            1.0,
            SystemTime::now(),
        );

        // not enough orders to calculate
        assert_eq!(orderbook.current_spread(), None);

        let order2 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            12.0,
            0.5,
            SystemTime::now(),
        );

        let order3 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Ask,
            12.5,
            2.5,
            SystemTime::now(),
        );

        orderbook.process_order(order1);
        orderbook.process_order(order2);
        orderbook.process_order(order3);

        assert_eq!(orderbook.current_spread(), Some((10.0, 12.0)));

        // wider spread
        let order4 = orders::new_limit_order_request(
            order_asset,
            price_asset,
            domain::OrderSide::Bid,
            14.0,
            1.5,
            SystemTime::now(),
        );
        let res = orderbook.process_order(order4);
        println!("{:?}", res);

        assert_eq!(orderbook.current_spread(), Some((10.0, 12.5)));
    }
}
